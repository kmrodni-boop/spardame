use crate::ai::{choose_pass_cards, choose_play, think_delay_ms};
use crate::cards::*;
use crate::engine::*;
use crate::i18n::{strings, Copy};
use crate::storage::{self, Settings, Stats};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Screen {
    Menu,
    Table,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Flash {
    CannotPassQueen,
    PickCards(usize),
    GotCards(usize, String),
    Illegal,
    PassFailed,
}

pub struct App {
    pub screen: Screen,
    pub settings: Settings,
    pub stats: Stats,
    pub state: Option<GameState>,
    pub selected: Vec<u8>,
    pub flash: Option<Flash>,
    pub rules_open: bool,
    pub score_open: bool,
    rng: Rng,
}

impl App {
    pub fn new() -> Self {
        let (settings, stats, game) = storage::load_save();
        let screen = if game.is_some() { Screen::Table } else { Screen::Menu };
        Self {
            screen,
            settings,
            stats,
            state: game,
            selected: Vec::new(),
            flash: None,
            rules_open: false,
            score_open: false,
            rng: Rng::new(),
        }
    }

    pub fn copy(&self) -> Copy {
        strings(self.settings.locale)
    }

    pub fn persist(&self) {
        let game = match self.screen {
            Screen::Table => self.state.as_ref(),
            Screen::Menu => match &self.state {
                Some(s) if s.phase == Phase::GameOver => None,
                other => other.as_ref(),
            },
        };
        storage::write_save(&self.settings, &self.stats, game);
    }

    pub fn set_settings(&mut self, patch: SettingsPatch) {
        if let Some(v) = patch.variant {
            self.settings.variant = v;
        }
        if let Some(d) = patch.difficulty {
            self.settings.difficulty = d;
        }
        if let Some(n) = patch.player_name {
            self.settings.player_name = n;
        }
        if let Some(a) = patch.ai_names {
            self.settings.ai_names = a;
        }
        if let Some(m) = patch.muted {
            self.settings.muted = m;
        }
        if let Some(l) = patch.locale {
            self.settings.locale = l;
        }
        if let Some(c) = patch.card_back {
            self.settings.card_back = c;
        }
        self.persist();
    }

    pub fn new_game(&mut self, variant: Option<VariantId>) {
        let v = variant.unwrap_or(self.settings.variant);
        if let Some(vd) = variant {
            if vd != self.settings.variant {
                self.settings.variant = vd;
            }
        }
        let raw = create_game(v, &mut self.rng);
        let state = fill_ai_passes(raw, self.settings.difficulty, &mut self.rng);
        self.screen = Screen::Table;
        self.state = Some(state);
        self.selected.clear();
        self.flash = None;
        self.score_open = false;
        self.persist();
    }

    pub fn continue_game(&mut self) {
        if self.state.is_none() {
            return;
        }
        let open = matches!(
            self.state.as_ref().map(|s| s.phase),
            Some(Phase::HandEnd) | Some(Phase::GameOver)
        );
        self.screen = Screen::Table;
        self.score_open = open;
    }

    pub fn abandon(&mut self) {
        self.screen = Screen::Menu;
        self.selected.clear();
        self.flash = None;
        self.score_open = false;
        self.persist();
    }

    pub fn toggle_card(&mut self, id: u8) {
        let Some(state) = &self.state else { return };
        if state.phase != Phase::Passing {
            return;
        }
        let v = get_variant(state.variant);
        if self.selected.contains(&id) {
            self.selected.retain(|x| *x != id);
            self.flash = None;
            return;
        }
        if self.selected.len() >= v.pass_count {
            return;
        }
        let card = match state.hands[0].iter().find(|c| c.id == id) {
            Some(c) => *c,
            None => return,
        };
        if !v.can_pass_queen && is_queen_of_spades(card) {
            self.flash = Some(Flash::CannotPassQueen);
            return;
        }
        self.selected.push(id);
        self.flash = None;
    }

    pub fn confirm_pass(&mut self) {
        let Some(state) = self.state.clone() else { return };
        if state.phase != Phase::Passing {
            return;
        }
        let v = get_variant(state.variant);
        if self.selected.len() != v.pass_count {
            self.flash = Some(Flash::PickCards(v.pass_count));
            return;
        }
        let cards: Vec<Card> = self
            .selected
            .iter()
            .map(|id| state.hands[0].iter().find(|c| c.id == *id).copied().unwrap())
            .collect();
        match set_pass_selection(&state, 0, &cards)
            .and_then(|s| commit_pass(&s))
        {
            Ok(next) => {
                let from = pass_from_name(&next, &self.settings);
                self.state = Some(next);
                self.selected.clear();
                self.flash = from.map(|f| Flash::GotCards(v.pass_count, f));
                self.persist();
            }
            Err(_) => {
                self.flash = Some(Flash::PassFailed);
            }
        }
    }

    pub fn play(&mut self, id: u8) -> bool {
        let Some(state) = self.state.clone() else { return false };
        if state.phase != Phase::Playing || state.turn != 0 {
            return false;
        }
        match play_card(&state, 0, id) {
            Ok(next) => {
                self.state = Some(next);
                self.selected.clear();
                self.flash = None;
                self.persist();
                true
            }
            Err(_) => {
                self.flash = Some(Flash::Illegal);
                false
            }
        }
    }

    pub fn play_ai(&mut self) -> bool {
        let Some(state) = self.state.clone() else { return false };
        if state.phase != Phase::Playing || state.turn == 0 {
            return false;
        }
        let card = choose_play(&state, state.turn, self.settings.difficulty);
        match play_card(&state, state.turn, card.id) {
            Ok(next) => {
                self.state = Some(next);
                self.flash = None;
                self.persist();
                true
            }
            Err(_) => false,
        }
    }

    pub fn settle_trick(&mut self) {
        let Some(state) = self.state.clone() else { return };
        if state.phase != Phase::TrickEnd {
            return;
        }
        let next = resolve_trick(&state);
        let mut next_stats = self.stats.clone();
        if next.phase == Phase::GameOver {
            let won = next.tied.contains(&0);
            next_stats.games_played = self.stats.games_played + 1;
            next_stats.games_won = self.stats.games_won + if won { 1 } else { 0 };
        }
        self.state = Some(next.clone());
        self.stats = next_stats;
        self.score_open = matches!(next.phase, Phase::HandEnd | Phase::GameOver);
        self.persist();
    }

    pub fn next_hand(&mut self) {
        let Some(state) = self.state.clone() else { return };
        if state.phase != Phase::HandEnd {
            return;
        }
        let raw = continue_game(&state, &mut self.rng);
        let filled = fill_ai_passes(raw, self.settings.difficulty, &mut self.rng);
        self.state = Some(filled);
        self.selected.clear();
        self.flash = None;
        self.score_open = false;
        self.persist();
    }

    pub fn set_rules_open(&mut self, open: bool) {
        self.rules_open = open;
    }

    pub fn set_score_open(&mut self, open: bool) {
        self.score_open = open;
    }

    pub fn clear_flash(&mut self) {
        self.flash = None;
    }

    pub fn think_delay(&self, legal_count: usize) -> u32 {
        think_delay_ms(self.settings.difficulty, legal_count)
    }

    pub fn seat_name(&self, id: PlayerId) -> String {
        if id == 0 {
            let n = self.settings.player_name.trim();
            if n.is_empty() {
                return self.copy().you.into();
            }
            return n.into();
        }
        self.settings.ai_names.get(id - 1).cloned().unwrap_or_else(|| self.copy().opponents.into())
    }

    pub fn names(&self) -> [String; 4] {
        [self.seat_name(0), self.seat_name(1), self.seat_name(2), self.seat_name(3)]
    }
}

fn fill_ai_passes(mut state: GameState, difficulty: Difficulty, _rng: &mut Rng) -> GameState {
    if state.phase != Phase::Passing {
        return state;
    }
    let mut current = state.clone();
    for p in 1..4 {
        let cards = choose_pass_cards(&current, p, difficulty);
        if let Ok(s) = set_pass_selection(&current, p, &cards) {
            current = s;
        }
    }
    state = current;
    state
}

fn pass_from_name(state: &GameState, settings: &Settings) -> Option<String> {
    match state.pass_dir {
        PassDir::Hold => None,
        PassDir::Left => Some(settings.ai_names[2].clone()),
        PassDir::Right => Some(settings.ai_names[0].clone()),
        PassDir::Across => Some(settings.ai_names[1].clone()),
    }
}

#[derive(Debug, Default, Clone)]
pub struct SettingsPatch {
    pub variant: Option<VariantId>,
    pub difficulty: Option<Difficulty>,
    pub player_name: Option<String>,
    pub ai_names: Option<[String; 3]>,
    pub muted: Option<bool>,
    pub locale: Option<Locale>,
    pub card_back: Option<CardBackTint>,
}

pub type AppRef = Rc<RefCell<App>>;
