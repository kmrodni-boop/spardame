use crate::app::{App, AppRef, Flash, Screen, SettingsPatch};
use crate::card_widget::{CardSize, CardWidget};
use crate::cards::*;
use crate::engine::*;
use crate::i18n::Copy;
use gtk::glib::timeout_add_local_once;
use gtk::prelude::*;
use gtk::{
    Align, Box as GtkBox, Button, CssProvider, Entry, Grid, Label, Orientation, Overlay,
    PolicyType, ScrolledWindow, Stack, ToggleButton,
};

use libadwaita::prelude::*;
use libadwaita::{Application, ApplicationWindow, HeaderBar};
use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

const CSS: &str = include_str!("styles.css");

struct MenuUi {
    root: gtk::Box,
    app_name: Label,
    tagline: Label,
    blurb: Label,
    name_label: Label,
    name_entry: Entry,
    diff_label: Label,
    back_label: Label,
    lang_label: Label,
    stats_label: Label,
    card_game_label: Label,
    variant_btns: [ToggleButton; 2],
    diff_btns: [ToggleButton; 3],
    back_btns: [ToggleButton; 2],
    lang_btns: [ToggleButton; 2],
    continue_btn: Button,
    sound_btn: Button,
    new_game_btn: Button,
    rules_btn: Button,
    hero_box: GtkBox,
}

struct TableUi {
    root: gtk::Box,
    seats_grid: Grid,
    hand_box: GtkBox,
    status_line: Label,
    confirm_pass: Button,
    scores_box: GtkBox,
    sound_btn: Button,
    lang_btn: Button,
    header: HeaderBar,
    overlay: Overlay,
}

struct Ui {
    stack: Stack,
    menu: MenuUi,
    table: TableUi,
    dialog_host: gtk::Box,
}

pub fn activate(app: &Application) {
    // CSS styling (commented out for now - needs display)
    // let provider = CssProvider::new();
    // provider.load_from_string(CSS);
    // if let Some(display) = gtk::gdk::Display::default() {
    //     gtk::style_context_add_provider_for_display(
    //         &display,
    //         &provider,
    //         gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    //     );
    // }

    let app_state: AppRef = Rc::new(RefCell::new(App::new()));
    let ui = Rc::new(RefCell::new(build_ui(app_state.clone())));

    let win = ApplicationWindow::builder()
        .application(app)
        .title("Spardame")
        .default_width(1100)
        .default_height(760)
        .width_request(380)
        .height_request(560)
        .content(&ui.borrow().dialog_host)
        .build();

    let state = app_state.clone();
    win.connect_close_request(move |_| {
        state.borrow().persist();
        gtk::glib::Propagation::Proceed
    });

    win.present();
    refresh(app_state, ui.clone());
}

fn build_ui(state: AppRef) -> Ui {
    let stack = Stack::builder().build();
    let menu = build_menu(state.clone());
    let table = build_table(state.clone());
    stack.add_named(&menu.root, Some("menu"));
    stack.add_named(&table.root, Some("table"));

    let dialog_host = GtkBox::builder()
        .orientation(Orientation::Vertical)
        .hexpand(true)
        .vexpand(true)
        .build();
    dialog_host.append(&stack);

    Ui {
        stack,
        menu,
        table,
        dialog_host,
    }
}

fn build_menu(state: AppRef) -> MenuUi {
    let root = GtkBox::builder()
        .orientation(Orientation::Vertical)
        .vexpand(true)
        .hexpand(true)
        .css_classes(["menu-screen"])
        .build();
    let header = HeaderBar::builder()
        .show_start_title_buttons(false)
        .show_end_title_buttons(false)
        .build();
    root.append(&header);

    let scroll = ScrolledWindow::builder()
        .hscrollbar_policy(PolicyType::Never)
        .vexpand(true)
        .hexpand(true)
        .build();
    let col = GtkBox::builder()
        .orientation(Orientation::Vertical)
        .spacing(14)
        .margin_start(28)
        .margin_end(28)
        .margin_top(8)
        .margin_bottom(28)
        .halign(Align::Center)
        .valign(Align::Center)
        .width_request(440)
        .build();
    scroll.set_child(Some(&col));
    root.append(&scroll);

    let card_game_label = Label::builder()
        .halign(Align::Start)
        .css_classes(["overline", "dim-label"])
        .build();
    col.append(&card_game_label);

    let app_name = Label::builder()
        .halign(Align::Start)
        .css_classes(["app-title"])
        .build();
    col.append(&app_name);

    let tagline = Label::builder()
        .wrap(true)
        .halign(Align::Start)
        .xalign(0.0)
        .css_classes(["dim-label", "tagline"])
        .build();
    col.append(&tagline);

    let hero_box = GtkBox::builder()
        .orientation(Orientation::Vertical)
        .halign(Align::Center)
        .margin_top(4)
        .margin_bottom(4)
        .build();
    col.append(&hero_box);

    let variant_grid = Grid::builder()
        .column_spacing(10)
        .row_spacing(10)
        .hexpand(true)
        .build();
    col.append(&variant_grid);
    let mut variant_btns = Vec::new();
    for &id in &[VariantId::Spardame, VariantId::Hjerter] {
        let btn = ToggleButton::builder().hexpand(true).build();
        let s = state.clone();
        btn.connect_clicked(move |_| {
            s.borrow_mut().set_settings(SettingsPatch {
                variant: Some(id),
                ..Default::default()
            });
        });
        variant_grid.attach(&btn, if matches!(id, VariantId::Spardame) { 0 } else { 1 }, 0, 1, 1);
        variant_btns.push(btn);
    }

    let blurb = Label::builder()
        .wrap(true)
        .xalign(0.0)
        .halign(Align::Start)
        .css_classes(["dim-label", "small"])
        .build();
    col.append(&blurb);

    let name_label = Label::builder()
        .halign(Align::Start)
        .css_classes(["dim-label", "small"])
        .build();
    col.append(&name_label);

    let name_entry = Entry::builder()
        .hexpand(true)
        .max_length(16)
        .css_classes(["name-entry"])
        .build();
    {
        let s = state.clone();
        name_entry.connect_changed(move |e| {
            let text = e.text().to_string();
            s.borrow_mut().set_settings(SettingsPatch {
                player_name: Some(text),
                ..Default::default()
            });
        });
    }
    col.append(&name_entry);

    let diff_label = Label::builder()
        .halign(Align::Start)
        .css_classes(["dim-label", "small"])
        .build();
    col.append(&diff_label);
    let diff_box = GtkBox::builder()
        .orientation(Orientation::Horizontal)
        .spacing(8)
        .homogeneous(true)
        .build();
    col.append(&diff_box);
    let mut diff_btns = Vec::new();
    for &d in &[Difficulty::Easy, Difficulty::Normal, Difficulty::Hard] {
        let btn = ToggleButton::builder().hexpand(true).build();
        let s = state.clone();
        btn.connect_clicked(move |_| {
            s.borrow_mut().set_settings(SettingsPatch {
                difficulty: Some(d),
                ..Default::default()
            });
        });
        diff_box.append(&btn);
        diff_btns.push(btn);
    }

    let back_label = Label::builder()
        .halign(Align::Start)
        .css_classes(["dim-label", "small"])
        .build();
    col.append(&back_label);
    let back_box = GtkBox::builder()
        .orientation(Orientation::Horizontal)
        .spacing(8)
        .homogeneous(true)
        .build();
    col.append(&back_box);
    let mut back_btns = Vec::new();
    for &tint in &[CardBackTint::Red, CardBackTint::Blue] {
        let btn = ToggleButton::builder().hexpand(true).build();
        let s = state.clone();
        btn.connect_clicked(move |_| {
            s.borrow_mut().set_settings(SettingsPatch {
                card_back: Some(tint),
                ..Default::default()
            });
        });
        back_box.append(&btn);
        back_btns.push(btn);
    }

    let lang_label = Label::builder()
        .halign(Align::Start)
        .css_classes(["dim-label", "small"])
        .build();
    col.append(&lang_label);
    let lang_box = GtkBox::builder()
        .orientation(Orientation::Horizontal)
        .spacing(8)
        .homogeneous(true)
        .build();
    col.append(&lang_box);
    let mut lang_btns = Vec::new();
    for &loc in &[Locale::En, Locale::Nb] {
        let btn = ToggleButton::builder().hexpand(true).build();
        let s = state.clone();
        btn.connect_clicked(move |_| {
            s.borrow_mut().set_settings(SettingsPatch {
                locale: Some(loc),
                ..Default::default()
            });
        });
        lang_box.append(&btn);
        lang_btns.push(btn);
    }

    let new_game_btn = Button::builder()
        .css_classes(["suggested", "pill", "cta"])
        .hexpand(true)
        .build();
    {
        let s = state.clone();
        new_game_btn.connect_clicked(move |_| {
            s.borrow_mut().new_game(None);
        });
    }
    col.append(&new_game_btn);

    let continue_btn = Button::builder()
        .css_classes(["pill"])
        .hexpand(true)
        .build();
    {
        let s = state.clone();
        continue_btn.connect_clicked(move |_| {
            s.borrow_mut().continue_game();
        });
    }
    col.append(&continue_btn);

    let action_row = GtkBox::builder()
        .orientation(Orientation::Horizontal)
        .spacing(10)
        .homogeneous(true)
        .build();
    let rules_btn = Button::builder().css_classes(["flat", "pill"]).build();
    let sound_btn = Button::builder().css_classes(["flat", "pill"]).build();
    {
        let s = state.clone();
        sound_btn.connect_clicked(move |_| {
            let muted = !s.borrow().settings.muted;
            s.borrow_mut().set_settings(SettingsPatch {
                muted: Some(muted),
                ..Default::default()
            });
        });
    }
    action_row.append(&rules_btn);
    action_row.append(&sound_btn);
    col.append(&action_row);

    let stats_label = Label::builder()
        .css_classes(["dim-label", "small"])
        .halign(Align::Start)
        .build();
    col.append(&stats_label);

    MenuUi {
        root,
        app_name,
        tagline,
        blurb,
        name_label,
        name_entry,
        diff_label,
        back_label,
        lang_label,
        stats_label,
        card_game_label,
        variant_btns: [variant_btns.remove(0), variant_btns.remove(0)],
        diff_btns: [diff_btns.remove(0), diff_btns.remove(0), diff_btns.remove(0)],
        back_btns: [back_btns.remove(0), back_btns.remove(0)],
        lang_btns: [lang_btns.remove(0), lang_btns.remove(0)],
        continue_btn,
        sound_btn,
        new_game_btn,
        rules_btn,
        hero_box,
    }
}

fn build_table(state: AppRef) -> TableUi {
    let root = GtkBox::builder()
        .orientation(Orientation::Vertical)
        .css_classes(["table-screen"])
        .vexpand(true)
        .hexpand(true)
        .build();
    let header = HeaderBar::builder()
        .show_start_title_buttons(false)
        .show_end_title_buttons(false)
        .build();
    root.append(&header);

    let scores_box = GtkBox::builder()
        .orientation(Orientation::Horizontal)
        .spacing(8)
        .halign(Align::Fill)
        .margin_start(10)
        .margin_end(10)
        .build();
    header.set_title_widget(Some(&scores_box));

    let menu_btn = Button::from_icon_name("go-home-symbolic");
    {
        let s = state.clone();
        menu_btn.connect_clicked(move |_| {
            s.borrow_mut().abandon();
        });
    }
    header.pack_start(&menu_btn);

    let sound_btn = Button::from_icon_name("audio-volume-high-symbolic");
    {
        let s = state.clone();
        sound_btn.connect_clicked(move |_| {
            let muted = !s.borrow().settings.muted;
            s.borrow_mut().set_settings(SettingsPatch {
                muted: Some(muted),
                ..Default::default()
            });
        });
    }
    let rules_btn = Button::from_icon_name("help-about-symbolic");
    {
        let s = state.clone();
        rules_btn.connect_clicked(move |_| {
            s.borrow_mut().set_rules_open(true);
        });
    }
    let lang_btn = Button::builder().css_classes(["flat"]).label("EN").build();
    {
        let s = state.clone();
        lang_btn.connect_clicked(move |_| {
            let next = if s.borrow().settings.locale == Locale::En {
                Locale::Nb
            } else {
                Locale::En
            };
            s.borrow_mut().set_settings(SettingsPatch {
                locale: Some(next),
                ..Default::default()
            });
        });
    }
    header.pack_end(&rules_btn);
    header.pack_end(&sound_btn);
    header.pack_end(&lang_btn);

    let overlay = Overlay::builder().css_classes(["felt"]).vexpand(true).hexpand(true).build();

    let seats_grid = Grid::builder()
        .column_spacing(10)
        .row_spacing(10)
        .halign(Align::Fill)
        .valign(Align::Fill)
        .hexpand(true)
        .vexpand(true)
        .margin_start(20)
        .margin_end(20)
        .margin_top(10)
        .margin_bottom(160)
        .build();
    overlay.set_child(Some(&seats_grid));
    root.append(&overlay);

    let status_line = Label::builder()
        .css_classes(["status-line"])
        .wrap(true)
        .justify(gtk::Justification::Center)
        .halign(Align::Center)
        .margin_top(6)
        .build();
    root.append(&status_line);

    let confirm_pass = Button::builder()
        .css_classes(["suggested", "pill"])
        .halign(Align::Center)
        .margin_top(6)
        .margin_bottom(4)
        .build();
    {
        let s = state.clone();
        confirm_pass.connect_clicked(move |_| {
            s.borrow_mut().confirm_pass();
        });
    }
    root.append(&confirm_pass);

    let hand_box = GtkBox::builder()
        .orientation(Orientation::Horizontal)
        .halign(Align::Center)
        .valign(Align::End)
        .margin_bottom(10)
        .css_classes(["hand-dock"])
        .build();
    root.append(&hand_box);

    TableUi {
        root,
        seats_grid,
        hand_box,
        status_line,
        confirm_pass,
        scores_box,
        sound_btn,
        lang_btn,
        header,
        overlay,
    }
}

fn refresh(state: AppRef, ui: Rc<RefCell<Ui>>) {
    let app = state.borrow();
    let copy = app.copy();
    let screen = app.screen;

    {
        let u = ui.borrow();
        match screen {
            Screen::Menu => u.stack.set_visible_child_name("menu"),
            Screen::Table => u.stack.set_visible_child_name("table"),
        }
    }

    update_menu(&app, &copy, &ui.borrow().menu);

    if app.state.is_some() {
        update_table(&app, &copy, &ui.borrow().table, state.clone(), ui.clone());
        schedule_ai(state.clone(), ui.clone());
    }

    let rules_open = app.rules_open;
    let score_open = app.score_open
        && app
            .state
            .as_ref()
            .map_or(false, |s| matches!(s.phase, Phase::HandEnd | Phase::GameOver));
    drop(app);

    if rules_open {
        show_rules(state.clone(), ui.clone());
    }
    if score_open {
        show_score(state.clone(), ui.clone());
    }
}

fn update_menu(app: &App, copy: &Copy, m: &MenuUi) {
    m.card_game_label.set_label(copy.card_game);
    m.app_name.set_label(copy.app_name);
    m.tagline.set_label(copy.tagline);
    m.blurb.set_label(copy.variant_blurb(app.settings.variant));
    m.name_label.set_label(copy.your_name);
    if m.name_entry.text().as_str() != app.settings.player_name.as_str() {
        m.name_entry.set_text(app.settings.player_name.as_str());
    }
    if app.settings.player_name.is_empty() {
        m.name_entry.set_placeholder_text(Some(copy.you));
    }
    m.diff_label.set_label(copy.opponents);
    m.back_label.set_label(copy.card_back);
    m.lang_label.set_label(copy.language);

    for (i, id) in [VariantId::Spardame, VariantId::Hjerter].iter().enumerate() {
        let btn = &m.variant_btns[i];
        btn.set_label(copy.variant_name(*id));
        let active = app.settings.variant == *id;
        if btn.is_active() != active {
            btn.set_active(active);
        }
        let extra = format!(
            "{} · {}",
            copy.variant_short(*id),
            copy.to_limit(get_variant(*id).game_limit)
        );
        btn.set_tooltip_text(Some(&extra));
    }
    for (i, d) in [Difficulty::Easy, Difficulty::Normal, Difficulty::Hard].iter().enumerate() {
        let btn = &m.diff_btns[i];
        btn.set_label(match d {
            Difficulty::Easy => copy.easy,
            Difficulty::Normal => copy.normal,
            Difficulty::Hard => copy.hard,
        });
        let active = app.settings.difficulty == *d;
        if btn.is_active() != active {
            btn.set_active(active);
        }
    }
    for (i, tint) in [CardBackTint::Red, CardBackTint::Blue].iter().enumerate() {
        let btn = &m.back_btns[i];
        btn.set_label(match tint {
            CardBackTint::Red => copy.card_back_tint_red,
            CardBackTint::Blue => copy.card_back_tint_blue,
        });
        let active = app.settings.card_back == *tint;
        if btn.is_active() != active {
            btn.set_active(active);
        }
    }
    for (i, loc) in [Locale::En, Locale::Nb].iter().enumerate() {
        let btn = &m.lang_btns[i];
        btn.set_label(match loc {
            Locale::En => "EN",
            Locale::Nb => "Norsk",
        });
        let active = app.settings.locale == *loc;
        if btn.is_active() != active {
            btn.set_active(active);
        }
    }

    m.new_game_btn.set_label(copy.new_game);
    m.continue_btn.set_label(copy.continue_);
    m.continue_btn.set_visible(
        app.state.is_some()
            && app.state.as_ref().map_or(false, |s| s.phase != Phase::GameOver),
    );
    m.rules_btn.set_label(copy.rules);
    m.sound_btn.set_label(if app.settings.muted { copy.sound_off } else { copy.sound_on });

    if app.stats.games_played > 0 {
        m.stats_label.set_label(&copy.games_won(app.stats.games_won, app.stats.games_played));
    } else {
        m.stats_label.set_label("");
    }

    clear_container(&m.hero_box);
    let hero = CardWidget::new_back(CardSize::Md, matches!(app.settings.card_back, CardBackTint::Red));
    hero.set_margin_top(8);
    hero.set_margin_bottom(8);
    m.hero_box.append(&hero);
}

fn update_table(app: &App, copy: &Copy, t: &TableUi, state: AppRef, _ui: Rc<RefCell<Ui>>) {
    let Some(s) = &app.state else { return };
    let names = app.names();

    clear_container(&t.scores_box);
    for p in 0..4 {
        let active = s.turn == p && s.phase == Phase::Playing;
        let chip = Label::builder()
            .label(&format!("{} {}", names[p], s.scores[p]))
            .css_classes(["score-chip", if active { "score-chip-active" } else { "score-chip-idle" }])
            .build();
        t.scores_box.append(&chip);
    }

    let muted_icon = if app.settings.muted { "audio-volume-muted-symbolic" } else { "audio-volume-high-symbolic" };
    t.sound_btn.set_icon_name(muted_icon);
    t.lang_btn.set_label(if app.settings.locale == Locale::En { "EN" } else { "NO" });

    clear_grid(&t.seats_grid);
    attach_seat(t, 2, &names[2], s, copy, app, 1, 0);
    attach_seat(t, 1, &names[1], s, copy, app, 0, 1);
    attach_seat(t, 3, &names[3], s, copy, app, 2, 1);

    let center = GtkBox::builder()
        .orientation(Orientation::Vertical)
        .halign(Align::Center)
        .valign(Align::Center)
        .hexpand(true)
        .vexpand(true)
        .build();
    if s.trick.is_empty() {
        let lead = suit_led(s);
        let txt = status_glyph(s, lead, copy);
        let lab = Label::builder().label(&txt).css_classes(["status-label"]).build();
        center.append(&lab);
    } else {
        let trick_row = GtkBox::builder()
            .orientation(Orientation::Horizontal)
            .halign(Align::Center)
            .valign(Align::Center)
            .build();
        let winner_now = current_winner_of_trick(&s.trick);
        for play in &s.trick {
            let winning = winner_now == Some(play.player) && s.trick.len() > 1;
            let cw = CardWidget::new_face(play.card, copy.face_letter(play.card), CardSize::Xl);
            cw.set_margin_start(18);
            cw.set_margin_end(18);
            if winning {
                cw.set_highlight(true);
            }
            trick_row.append(&cw);
        }
        center.append(&trick_row);
    }
    t.seats_grid.attach(&center, 1, 1, 1, 1);

    let status = match &app.flash {
        Some(f) => flash_text(copy, f),
        None => status_text(s, &names, copy),
    };
    t.status_line.set_label(&status);

    let passing = s.phase == Phase::Passing;
    t.confirm_pass.set_visible(passing);
    if passing {
        let v = get_variant(s.variant);
        t.confirm_pass.set_label(&copy.send_cards(app.selected.len(), v.pass_count, copy.pass_dir(s.pass_dir)));
        t.confirm_pass.set_sensitive(app.selected.len() == v.pass_count);
    }

    clear_container(&t.hand_box);
    let hand = &s.hands[0];
    let legal = if s.phase == Phase::Playing && s.turn == 0 {
        legal_moves(s, 0)
    } else {
        Vec::new()
    };
    let legal_ids: std::collections::HashSet<u8> = legal.iter().map(|c| c.id).collect();
    let my_turn = s.phase == Phase::Playing && s.turn == 0;
    let received: std::collections::HashSet<u8> = s.received_pass.iter().map(|c| c.id).collect();

    for card in hand {
        let cw = CardWidget::new_face(*card, copy.face_letter(*card), CardSize::Lg);
        cw.set_margin_start(20);
        cw.set_margin_end(20);
        let is_sel = app.selected.contains(&card.id);
        cw.set_selected(is_sel);
        if my_turn && !legal_ids.contains(&card.id) {
            cw.set_dimmed(true);
        }
        if received.contains(&card.id) && s.trick_number == 0 && s.trick.is_empty() {
            cw.set_highlight(true);
        }
        let clickable = passing || my_turn;
        if clickable {
            let st = state.clone();
            let id = card.id;
            let gesture = gtk::GestureClick::new();
            gesture.connect_released(move |_, _, _, _| {
                {
                    let mut a = st.borrow_mut();
                    if a.state.as_ref().map_or(false, |s| s.phase == Phase::Passing) {
                        a.toggle_card(id);
                    } else {
                        a.play(id);
                    }
                }
            });
            cw.add_controller(gesture);
        }
        t.hand_box.append(&cw);
    }
}

fn attach_seat(
    t: &TableUi,
    player: PlayerId,
    name: &str,
    s: &GameState,
    _copy: &Copy,
    app: &App,
    col: i32,
    row: i32,
) {
    let col_box = GtkBox::builder()
        .orientation(Orientation::Vertical)
        .spacing(6)
        .halign(Align::Center)
        .valign(Align::Center)
        .build();
    let thinking = s.phase == Phase::Playing && s.turn == player;
    let thinking_label = if thinking { format!("{} …", name) } else { name.to_string() };
    let nm = Label::builder()
        .label(&thinking_label)
        .css_classes([
            "seat-name",
            if thinking { "seat-name-active" } else { "seat-name-idle" },
        ])
        .build();
    col_box.append(&nm);

    let backs = GtkBox::builder()
        .orientation(Orientation::Horizontal)
        .halign(Align::Center)
        .build();
    let count = s.hands[player].len();
    let show = count.min(6);
    let tint = matches!(app.settings.card_back, CardBackTint::Red);
    for _ in 0..show {
        let b = CardWidget::new_back(CardSize::Xs, tint);
        b.set_margin_start(-16);
        b.set_margin_end(-16);
        backs.append(&b);
    }
    col_box.append(&backs);

    let points = point_cards_taken(&s.taken[player], s.variant);
    if !points.is_empty() {
        let pstr: Vec<String> = points.iter().take(6).map(|c| suit_glyph_str(c.suit())).collect();
        let plab = Label::builder()
            .label(&pstr.join(" "))
            .css_classes(["dim-label", "small"])
            .build();
        col_box.append(&plab);
    }

    t.seats_grid.attach(&col_box, col, row, 1, 1);
}

fn suit_glyph_str(suit: u8) -> String {
    match suit {
        CLUBS => "♣".into(),
        DIAMONDS => "♦".into(),
        SPADES => "♠".into(),
        HEARTS => "♥".into(),
        _ => "".into(),
    }
}

fn status_glyph(s: &GameState, lead: Option<u8>, copy: &Copy) -> String {
    if s.phase == Phase::Passing {
        return copy.passing.into();
    }
    if let Some(suit) = lead {
        return format!("{} · {}", suit_glyph_str(suit), copy.suit_cap(suit));
    }
    let mut bits = Vec::new();
    if s.hearts_broken {
        bits.push(copy.hearts_broken);
    }
    if queen_still_out(s) {
        bits.push(copy.queen_out);
    }
    if matches!(s.variant, VariantId::Spardame) && jack_still_out(s) {
        bits.push(copy.jack_out);
    }
    if bits.is_empty() {
        return copy.lead.into();
    }
    bits.join(" · ")
}

fn status_text(s: &GameState, names: &[String; 4], copy: &Copy) -> String {
    if s.phase == Phase::Passing {
        return copy.pick_cards(get_variant(s.variant).pass_count, copy.pass_dir(s.pass_dir));
    }
    if s.phase == Phase::TrickEnd {
        return match current_winner_of_trick(&s.trick) {
            None => copy.trick.into(),
            Some(w) => copy.takes_trick(&names[w], w == 0),
        };
    }
    if s.phase == Phase::HandEnd {
        return copy.hand_over.into();
    }
    if s.phase == Phase::GameOver {
        return copy.game_over.into();
    }
    if s.turn != 0 {
        return copy.thinking(&names[s.turn]);
    }
    if s.trick.is_empty() {
        if s.trick_number == 0 {
            return copy.your_lead_two.into();
        }
        return copy.your_lead.into();
    }
    let suit = s.trick[0].card.suit();
    let can_follow = s.hands[0].iter().any(|c| c.suit() == suit);
    if can_follow {
        return copy.follow_suit(copy.suit_name(suit));
    }
    copy.void_in(copy.suit_name(suit))
}

fn flash_text(copy: &Copy, f: &Flash) -> String {
    match f {
        Flash::CannotPassQueen => copy.cannot_pass_queen.into(),
        Flash::PickCards(n) => copy.pick_cards_short(*n),
        Flash::GotCards(n, from) => copy.got_cards(*n, from),
        Flash::Illegal => copy.illegal.into(),
        Flash::PassFailed => copy.pass_failed.into(),
    }
}

fn schedule_ai(state: AppRef, ui: Rc<RefCell<Ui>>) {
    let needs = {
        let a = state.borrow();
        a.state
            .as_ref()
            .map_or(false, |s| s.phase == Phase::Playing && s.turn != 0)
    };
    if !needs {
        return;
    }
    let legal_count = state
        .borrow()
        .state
        .as_ref()
        .map(|s| legal_moves(s, s.turn).len())
        .unwrap_or(0);
    let delay = state.borrow().think_delay(legal_count);
    let s = state.clone();
    let u = ui.clone();
    timeout_add_local_once(Duration::from_millis(delay.max(100) as u64), move || {
        {
            let mut a = s.borrow_mut();
            if a
                .state
                .as_ref()
                .map_or(false, |st| st.phase == Phase::Playing && st.turn != 0)
            {
                a.play_ai();
            }
        }
        refresh(s.clone(), u.clone());
        let needs_settle = s
            .borrow()
            .state
            .as_ref()
            .map_or(false, |st| st.phase == Phase::TrickEnd);
        if needs_settle {
            let s2 = s.clone();
            let u2 = u.clone();
            timeout_add_local_once(Duration::from_millis(900), move || {
                s2.borrow_mut().settle_trick();
                refresh(s2.clone(), u2.clone());
            });
        }
    });
}

fn show_rules(state: AppRef, ui: Rc<RefCell<Ui>>) {
    let app = state.borrow();
    let copy = app.copy();
    drop(app);
    let win = ui.borrow().dialog_host.root().and_downcast::<ApplicationWindow>();
    let Some(win) = win else { return };

    let dialog = libadwaita::Dialog::builder()
        .title(copy.rules_title)
        .content_width(560)
        .build();
    let content = GtkBox::builder()
        .orientation(Orientation::Vertical)
        .spacing(16)
        .margin_start(24)
        .margin_end(24)
        .margin_top(16)
        .margin_bottom(24)
        .build();

    let h = Label::builder().label(copy.rules_spardame_title).css_classes(["modal-title"]).halign(Align::Start).build();
    content.append(&h);
    content.append(&Label::builder().label(copy.rules_spardame_body).wrap(true).xalign(0.0).halign(Align::Start).build());
    for p in copy.rules_spardame_points.iter() {
        content.append(&Label::builder().label(&format!("• {}", p)).xalign(0.0).halign(Align::Start).build());
    }
    content.append(&Label::builder().label(copy.rules_spardame_extra).wrap(true).xalign(0.0).halign(Align::Start).css_classes(["dim-label", "small"]).build());

    let h2 = Label::builder().label(copy.rules_hearts_title).css_classes(["modal-title"]).halign(Align::Start).margin_top(8).build();
    content.append(&h2);
    content.append(&Label::builder().label(copy.rules_hearts_body).wrap(true).xalign(0.0).halign(Align::Start).build());

    let h3 = Label::builder().label(copy.rules_play_title).css_classes(["modal-title"]).halign(Align::Start).margin_top(8).build();
    content.append(&h3);
    for p in copy.rules_play.iter() {
        content.append(&Label::builder().label(&format!("• {}", p)).wrap(true).xalign(0.0).halign(Align::Start).build());
    }

    let scroller = ScrolledWindow::builder().vexpand(true).hscrollbar_policy(PolicyType::Never).build();
    scroller.set_child(Some(&content));
    dialog.set_child(Some(&scroller));

    let s = state.clone();
    let uc = ui.clone();
    dialog.connect_closed(move |_| {
        s.borrow_mut().set_rules_open(false);
        refresh(s.clone(), uc.clone());
    });
    let _ = dialog.present(Some(&win));
}

fn show_score(state: AppRef, ui: Rc<RefCell<Ui>>) {
    let app = state.borrow();
    let copy = app.copy();
    let s = match &app.state {
        Some(s) => s.clone(),
        None => return,
    };
    let names = app.names();
    let v = get_variant(s.variant);
    let over = s.phase == Phase::GameOver;
    let hs = s.hand_score.clone();
    let moon_name = hs.as_ref().and_then(|h| h.moon).map(|m| names[m].clone());
    let title = if over {
        if s.tied.len() > 1 {
            copy.draw.to_string()
        } else if s.winner == Some(0) {
            copy.you_won.to_string()
        } else {
            copy.won(&names[s.winner.unwrap_or(0)])
        }
    } else {
        copy.score_hand.to_string()
    };
    drop(app);

    let win = ui.borrow().dialog_host.root().and_downcast::<ApplicationWindow>();
    let Some(win) = win else { return };

    let dialog = libadwaita::Dialog::builder().title(&title).content_width(460).build();
    let content = GtkBox::builder()
        .orientation(Orientation::Vertical)
        .spacing(12)
        .margin_start(24)
        .margin_end(24)
        .margin_top(16)
        .margin_bottom(20)
        .build();

    if let Some(mn) = moon_name {
        content.append(&Label::builder().label(&copy.moon(&mn)).wrap(true).css_classes(["dim-label", "small"]).halign(Align::Start).build());
    }

    let grid = Grid::builder().column_spacing(12).row_spacing(2).build();
    let hdr_player = Label::builder().label(copy.player).halign(Align::Start).css_classes(["score-table-header"]).hexpand(true).build();
    let hdr_round = Label::builder().label(copy.round).halign(Align::End).css_classes(["score-table-header"]).build();
    let hdr_total = Label::builder().label(copy.total).halign(Align::End).css_classes(["score-table-header"]).build();
    grid.attach(&hdr_player, 0, 0, 1, 1);
    grid.attach(&hdr_round, 1, 0, 1, 1);
    grid.attach(&hdr_total, 2, 0, 1, 1);
    for p in 0..4 {
        let lowest = over && s.tied.contains(&p);
        let cls = if lowest { "score-table-row-winner" } else { "" };
        let nm = Label::builder().label(&names[p]).halign(Align::Start).css_classes(["score-table-row", cls]).build();
        let applied = hs.as_ref().map(|h| h.applied[p]).unwrap_or(0);
        let round_txt = if applied > 0 { format!("+{}", applied) } else { applied.to_string() };
        let rd = Label::builder().label(&round_txt).halign(Align::End).css_classes(["score-table-row", cls]).build();
        let total_txt = if s.scores[p] >= v.game_limit {
            format!("{} · {}", s.scores[p], copy.out)
        } else {
            s.scores[p].to_string()
        };
        let tot = Label::builder().label(&total_txt).halign(Align::End).css_classes(["score-table-row", cls]).build();
        grid.attach(&nm, 0, (p + 1) as i32, 1, 1);
        grid.attach(&rd, 1, (p + 1) as i32, 1, 1);
        grid.attach(&tot, 2, (p + 1) as i32, 1, 1);
    }
    content.append(&grid);
    content.append(&Label::builder().label(&copy.score_hint(v.game_limit)).css_classes(["dim-label", "small"]).halign(Align::Start).build());

    let actions = GtkBox::builder().orientation(Orientation::Horizontal).spacing(10).halign(Align::End).margin_top(8).build();
    let (menu_label, next_label, on_next) = if over {
        (copy.menu, copy.new_game, false)
    } else {
        (copy.quit, copy.next_hand, true)
    };
    let menu_btn = Button::builder().label(menu_label).css_classes(["flat"]).build();
    {
        let st = state.clone();
        let ui2 = ui.clone();
        menu_btn.connect_clicked(move |_| {
            st.borrow_mut().abandon();
            refresh(st.clone(), ui2.clone());
        });
    }
    let action_btn = Button::builder().label(next_label).css_classes(["suggested"]).build();
    {
        let st = state.clone();
        let ui3 = ui.clone();
        action_btn.connect_clicked(move |_| {
            if on_next {
                st.borrow_mut().next_hand();
            } else {
                st.borrow_mut().new_game(None);
            }
            refresh(st.clone(), ui3.clone());
        });
    }
    actions.append(&menu_btn);
    actions.append(&action_btn);
    content.append(&actions);

    dialog.set_child(Some(&content));

    let stc = state.clone();
    let uic = ui.clone();
    dialog.connect_closed(move |_| {
        stc.borrow_mut().set_score_open(false);
        refresh(stc.clone(), uic.clone());
    });
    let _ = dialog.present(Some(&win));
}

fn clear_container(container: &gtk::Box) {
    let mut next = container.first_child();
    while let Some(child) = next {
        next = child.next_sibling();
        container.remove(&child);
    }
}

fn clear_grid(grid: &Grid) {
    let mut next = grid.first_child();
    while let Some(child) = next {
        next = child.next_sibling();
        grid.remove(&child);
    }
}
