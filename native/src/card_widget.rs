use crate::cards::{Card, HEARTS, DIAMONDS, SPADES, CLUBS};
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gsk, graphene, Align, Overflow};
use std::cell::{Cell, RefCell};

const HEART_GLYPH: &str = "\u{2665}";
const DIAMOND_GLYPH: &str = "\u{2666}";
const SPADE_GLYPH: &str = "\u{2660}";
const CLUB_GLYPH: &str = "\u{2663}";

fn suit_glyph(suit: u8) -> &'static str {
    match suit {
        CLUBS => CLUB_GLYPH,
        DIAMONDS => DIAMOND_GLYPH,
        SPADES => SPADE_GLYPH,
        HEARTS => HEART_GLYPH,
        _ => "",
    }
}

fn is_red(suit: u8) -> bool {
    suit == HEARTS || suit == DIAMONDS
}

fn rgba(r: f32, g: f32, b: f32, a: f32) -> gtk::gdk::RGBA {
    gtk::gdk::RGBA::new(r, g, b, a)
}

fn rounded_rect(x: f32, y: f32, w: f32, h: f32, radius: f32) -> gsk::RoundedRect {
    let rect = graphene::Rect::new(x, y, w, h);
    let sz = graphene::Size::new(radius, radius);
    gsk::RoundedRect::new(rect, sz, sz, sz, sz)
}

fn rounded_path(x: f32, y: f32, w: f32, h: f32, radius: f32) -> gsk::Path {
    let r = rounded_rect(x, y, w, h, radius);
    let b = gsk::PathBuilder::new();
    b.add_rounded_rect(&r);
    b.to_path()
}

/// Pango absolute size is device pixels × PANGO_SCALE.
fn pango_px(px: f32) -> f64 {
    f64::from(px.max(5.0)) * f64::from(gtk::pango::SCALE)
}

fn corner_font_px(size: CardSize, letter: &str) -> f32 {
    let base = match size {
        CardSize::Xs => 7.5,
        CardSize::Sm => 10.0,
        CardSize::Md => 12.5,
        CardSize::Lg => 14.0,
        CardSize::Xl => 16.5,
    };
    if letter.chars().count() > 1 {
        base * 0.78
    } else {
        base
    }
}

fn pip_font_px(size: CardSize, ace: bool) -> f32 {
    if ace {
        match size {
            CardSize::Xs => 16.0,
            CardSize::Sm => 22.0,
            CardSize::Md => 30.0,
            CardSize::Lg => 36.0,
            CardSize::Xl => 46.0,
        }
    } else {
        match size {
            CardSize::Xs => 8.0,
            CardSize::Sm => 11.0,
            CardSize::Md => 13.5,
            CardSize::Lg => 15.5,
            CardSize::Xl => 18.0,
        }
    }
}

fn face_letter_px(size: CardSize, letter: &str) -> f32 {
    let base = match size {
        CardSize::Xs => 14.0,
        CardSize::Sm => 20.0,
        CardSize::Md => 26.0,
        CardSize::Lg => 30.0,
        CardSize::Xl => 40.0,
    };
    if letter.chars().count() > 1 {
        base * 0.78
    } else {
        base
    }
}

#[derive(Debug, Clone, Copy)]
pub enum CardKind {
    Face(Card),
    Back,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CardSize {
    Xs,
    Sm,
    Md,
    Lg,
    Xl,
}

fn card_width(size: CardSize) -> i32 {
    match size {
        CardSize::Xs => 38,
        CardSize::Sm => 56,
        CardSize::Md => 74,
        CardSize::Lg => 82,
        CardSize::Xl => 110,
    }
}

fn card_height(size: CardSize) -> i32 {
    (card_width(size) as f32 * 1.4) as i32
}

fn card_radius(size: CardSize) -> f32 {
    match size {
        CardSize::Xs => 5.0,
        CardSize::Sm => 6.0,
        CardSize::Md => 7.0,
        CardSize::Lg => 8.0,
        CardSize::Xl => 11.0,
    }
}

mod imp {
    use super::*;

    pub struct CardWidget {
        pub kind: RefCell<CardKind>,
        pub size: Cell<CardSize>,
        pub selected: Cell<bool>,
        pub dimmed: Cell<bool>,
        pub highlight: Cell<bool>,
        pub tint_red: Cell<bool>,
        pub face_letter: RefCell<String>,
        pub card_id: Cell<u8>,
    }

    impl Default for CardWidget {
        fn default() -> Self {
            Self {
                kind: RefCell::new(CardKind::Back),
                size: Cell::new(CardSize::Md),
                selected: Cell::new(false),
                dimmed: Cell::new(false),
                highlight: Cell::new(false),
                tint_red: Cell::new(true),
                face_letter: RefCell::new(String::new()),
                card_id: Cell::new(0),
            }
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for CardWidget {
        const NAME: &'static str = "SpardameCard";
        type Type = super::CardWidget;
        type ParentType = gtk::Widget;
    }

    impl ObjectImpl for CardWidget {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();
            obj.set_overflow(Overflow::Hidden);
            obj.set_halign(Align::Center);
            obj.set_valign(Align::Center);
        }
    }

    impl WidgetImpl for CardWidget {
        fn measure(&self, orientation: gtk::Orientation, _for_size: i32) -> (i32, i32, i32, i32) {
            let w = card_width(self.size.get());
            let h = card_height(self.size.get());
            match orientation {
                gtk::Orientation::Horizontal => (w, w, -1, -1),
                _ => (h, h, -1, -1),
            }
        }

        fn request_mode(&self) -> gtk::SizeRequestMode {
            gtk::SizeRequestMode::ConstantSize
        }

        fn snapshot(&self, snapshot: &gtk::Snapshot) {
            let obj = self.obj();
            let w = obj.width() as f32;
            let h = obj.height() as f32;
            if w < 1.0 || h < 1.0 {
                return;
            }
            let radius = card_radius(self.size.get());

            snapshot.push_rounded_clip(&rounded_rect(2.0, 3.0, w, h, radius));
            snapshot.append_color(
                &rgba(0.0, 0.0, 0.0, 0.28),
                &graphene::Rect::new(2.0, 3.0, w, h),
            );
            snapshot.pop();

            snapshot.push_rounded_clip(&rounded_rect(0.0, 0.0, w, h, radius));
            match *self.kind.borrow() {
                CardKind::Back => self.draw_back(snapshot, w, h, radius),
                CardKind::Face(card) => self.draw_face(snapshot, w, h, radius, card),
            }
            if self.selected.get() {
                snapshot.append_color(
                    &rgba(0.95, 0.92, 0.85, 0.45),
                    &graphene::Rect::new(0.0, 0.0, w, h),
                );
            }
            if self.highlight.get() {
                let path = rounded_path(0.0, 0.0, w, h, radius);
                snapshot.append_stroke(&path, &gsk::Stroke::new(2.5), &rgba(0.95, 0.92, 0.85, 0.95));
            }
            snapshot.pop();
        }
    }

    impl CardWidget {
        fn draw_back(&self, snapshot: &gtk::Snapshot, w: f32, h: f32, radius: f32) {
            let (deep, mid) = if self.tint_red.get() {
                (rgba(0.49, 0.08, 0.18, 1.0), rgba(0.69, 0.12, 0.27, 1.0))
            } else {
                (rgba(0.08, 0.21, 0.42, 1.0), rgba(0.12, 0.31, 0.59, 1.0))
            };
            let bounds = graphene::Rect::new(0.0, 0.0, w, h);
            snapshot.append_linear_gradient(
                &bounds,
                &graphene::Point::new(0.0, 0.0),
                &graphene::Point::new(0.0, h),
                &[gsk::ColorStop::new(0.0, deep), gsk::ColorStop::new(1.0, mid)],
            );

            let pad = w * 0.07;
            let ir = radius * 0.7;
            snapshot.push_rounded_clip(&rounded_rect(pad, pad, w - 2.0 * pad, h - 2.0 * pad, ir));
            let ib = graphene::Rect::new(pad, pad, w - 2.0 * pad, h - 2.0 * pad);
            snapshot.append_linear_gradient(
                &ib,
                &graphene::Point::new(0.0, pad),
                &graphene::Point::new(0.0, h - pad),
                &[gsk::ColorStop::new(0.0, mid), gsk::ColorStop::new(1.0, deep)],
            );

            let cream = rgba(0.95, 0.92, 0.85, 0.22);
            let step = (w - 2.0 * pad).max(8.0) / 5.5;
            let mut row = 0;
            let mut y = pad + step * 0.2;
            while y < h - pad {
                let offset = if row % 2 == 1 { step * 0.5 } else { 0.0 };
                let mut x = pad + step * 0.15 + offset;
                while x < w - pad {
                    snapshot.save();
                    snapshot.translate(&graphene::Point::new(x, y));
                    snapshot.rotate(45.0);
                    let s = step * 0.36;
                    snapshot.append_color(&cream, &graphene::Rect::new(-s * 0.5, -s * 0.5, s, s));
                    snapshot.restore();
                    x += step;
                }
                y += step * 0.52;
                row += 1;
            }

            let medal_r = (w.min(h) * 0.22).max(8.0);
            snapshot.push_rounded_clip(&rounded_rect(
                w * 0.5 - medal_r,
                h * 0.5 - medal_r,
                medal_r * 2.0,
                medal_r * 2.0,
                medal_r,
            ));
            snapshot.append_color(
                &rgba(0.95, 0.92, 0.85, 0.16),
                &graphene::Rect::new(w * 0.5 - medal_r, h * 0.5 - medal_r, medal_r * 2.0, medal_r * 2.0),
            );
            snapshot.pop();

            let obj = self.obj();
            let layout = obj.create_pango_layout(Some(SPADE_GLYPH));
            let mut desc = gtk::pango::FontDescription::from_string("Sans Bold");
            desc.set_absolute_size(pango_px(h * 0.22));
            layout.set_font_description(Some(&desc));
            let (lw, lh) = layout.pixel_size();
            snapshot.save();
            snapshot.translate(&graphene::Point::new(
                w / 2.0 - lw as f32 / 2.0,
                h / 2.0 - lh as f32 / 2.0,
            ));
            snapshot.append_layout(&layout, &rgba(0.95, 0.92, 0.85, 0.92));
            snapshot.restore();
            snapshot.pop();
        }

        fn draw_face(&self, snapshot: &gtk::Snapshot, w: f32, h: f32, radius: f32, card: Card) {
            let cream = rgba(1.0, 0.99, 0.97, 1.0);
            snapshot.append_color(&cream, &graphene::Rect::new(0.0, 0.0, w, h));

            let edge = if is_red(card.suit()) {
                rgba(0.70, 0.14, 0.17, 0.28)
            } else {
                rgba(0.85, 0.80, 0.71, 1.0)
            };
            let path = rounded_path(0.0, 0.0, w, h, radius);
            snapshot.append_stroke(&path, &gsk::Stroke::new(1.0), &edge);

            let suit = card.suit();
            let letter = self.face_letter.borrow().clone();
            let ink = if is_red(suit) {
                rgba(0.70, 0.14, 0.17, 1.0)
            } else {
                rgba(0.08, 0.07, 0.05, 1.0)
            };

            let obj = self.obj();
            self.draw_corner(&obj, snapshot, &letter, suit, w, h, true, &ink);
            self.draw_corner(&obj, snapshot, &letter, suit, w, h, false, &ink);

            let is_face = card.rank() >= 11;
            let is_ace = card.rank() == 14;
            if is_face {
                let pad_x = w * 0.18;
                let pad_y = h * 0.16;
                snapshot.push_rounded_clip(&rounded_rect(
                    pad_x,
                    pad_y,
                    w - 2.0 * pad_x,
                    h - 2.0 * pad_y,
                    radius * 0.55,
                ));
                let ib = graphene::Rect::new(pad_x, pad_y, w - 2.0 * pad_x, h - 2.0 * pad_y);
                snapshot.append_linear_gradient(
                    &ib,
                    &graphene::Point::new(pad_x, pad_y),
                    &graphene::Point::new(pad_x, h - pad_y),
                    &[
                        gsk::ColorStop::new(0.0, cream),
                        gsk::ColorStop::new(1.0, rgba(0.95, 0.91, 0.82, 1.0)),
                    ],
                );
                let ipath = rounded_path(pad_x, pad_y, w - 2.0 * pad_x, h - 2.0 * pad_y, radius * 0.55);
                snapshot.append_stroke(&ipath, &gsk::Stroke::new(1.0), &rgba(0.85, 0.80, 0.71, 0.9));
                snapshot.pop();

                let layout = obj.create_pango_layout(Some(&letter));
                let mut desc = gtk::pango::FontDescription::from_string("Serif");
                desc.set_weight(gtk::pango::Weight::Bold);
                desc.set_absolute_size(pango_px(face_letter_px(self.size.get(), &letter)));
                layout.set_font_description(Some(&desc));
                let (lw, lh) = layout.pixel_size();
                snapshot.save();
                snapshot.translate(&graphene::Point::new(
                    w / 2.0 - lw as f32 / 2.0,
                    h * 0.34,
                ));
                snapshot.append_layout(&layout, &ink);
                snapshot.restore();

                let layout = obj.create_pango_layout(Some(suit_glyph(suit)));
                let mut desc = gtk::pango::FontDescription::from_string("Sans");
                desc.set_absolute_size(pango_px(face_letter_px(self.size.get(), "A") * 0.42));
                layout.set_font_description(Some(&desc));
                let (lw, _) = layout.pixel_size();
                snapshot.save();
                snapshot.translate(&graphene::Point::new(w / 2.0 - lw as f32 / 2.0, h * 0.58));
                snapshot.append_layout(&layout, &ink);
                snapshot.restore();
                let _ = lh;

                self.draw_royal_mark(snapshot, card.rank(), w, h, &ink);
            } else {
                let pips = pip_positions(is_ace, card.rank());
                for &(px, py, flip) in pips.iter() {
                    self.draw_pip(&obj, snapshot, suit, w, h, px, py, flip, is_ace, &ink);
                }
            }
        }

        fn draw_corner(
            &self,
            obj: &super::CardWidget,
            snapshot: &gtk::Snapshot,
            letter: &str,
            suit: u8,
            w: f32,
            h: f32,
            top_left: bool,
            ink: &gtk::gdk::RGBA,
        ) {
            let px = corner_font_px(self.size.get(), letter);
            let layout = obj.create_pango_layout(Some(letter));
            let mut desc = gtk::pango::FontDescription::from_string("Serif");
            desc.set_weight(gtk::pango::Weight::Bold);
            desc.set_absolute_size(pango_px(px));
            layout.set_font_description(Some(&desc));
            let (lw, lh) = layout.pixel_size();

            let sl = obj.create_pango_layout(Some(suit_glyph(suit)));
            let mut sd = gtk::pango::FontDescription::from_string("Sans");
            sd.set_absolute_size(pango_px(px * 0.82));
            sl.set_font_description(Some(&sd));
            let (sw, _sh) = sl.pixel_size();

            snapshot.save();
            if !top_left {
                snapshot.translate(&graphene::Point::new(w, h));
                snapshot.rotate(180.0);
            }
            let x = w * 0.07;
            let y = h * 0.045;
            snapshot.save();
            snapshot.translate(&graphene::Point::new(x, y));
            snapshot.append_layout(&layout, ink);
            snapshot.restore();

            let gx = x + (lw as f32 - sw as f32) / 2.0;
            let gy = y + lh as f32 - 1.0;
            snapshot.save();
            snapshot.translate(&graphene::Point::new(gx, gy));
            snapshot.append_layout(&sl, ink);
            snapshot.restore();
            snapshot.restore();
        }

        // A small line-art motif under the letter+suit on face cards, drawn
        // with the same primitives as the back's lattice/medal (paths and
        // circles, no assets). Ace is excluded — it already renders as a
        // lettered face card, not a mark-bearing "royal" one.
        fn draw_royal_mark(&self, snapshot: &gtk::Snapshot, rank: u8, w: f32, h: f32, ink: &gtk::gdk::RGBA) {
            let cx = w / 2.0;
            let cy = h * 0.745;
            match rank {
                13 => self.draw_king_mark(snapshot, cx, cy, w, ink),
                12 => self.draw_queen_mark(snapshot, cx, cy, w, ink),
                11 => self.draw_jack_mark(snapshot, cx, cy, w, ink),
                _ => {}
            }
        }

        fn draw_king_mark(&self, snapshot: &gtk::Snapshot, cx: f32, cy: f32, w: f32, ink: &gtk::gdk::RGBA) {
            let s = w * 0.15;
            let base_y = cy + s * 0.5;
            let low_y = cy;
            let high_y = cy - s * 0.75;
            let b = gsk::PathBuilder::new();
            b.move_to(cx - s, base_y);
            b.line_to(cx - s, low_y);
            b.line_to(cx - s * 0.5, high_y);
            b.line_to(cx - s * 0.22, low_y);
            b.line_to(cx, high_y - s * 0.2);
            b.line_to(cx + s * 0.22, low_y);
            b.line_to(cx + s * 0.5, high_y);
            b.line_to(cx + s, low_y);
            b.line_to(cx + s, base_y);
            b.close();
            let fill = rgba(ink.red(), ink.green(), ink.blue(), 0.55);
            snapshot.append_fill(&b.to_path(), gsk::FillRule::Winding, &fill);

            let jb = gsk::PathBuilder::new();
            jb.add_circle(&graphene::Point::new(cx, high_y - s * 0.15), s * 0.1);
            snapshot.append_fill(&jb.to_path(), gsk::FillRule::Winding, ink);
        }

        fn draw_queen_mark(&self, snapshot: &gtk::Snapshot, cx: f32, cy: f32, w: f32, ink: &gtk::gdk::RGBA) {
            let s = w * 0.10;
            let fill = rgba(ink.red(), ink.green(), ink.blue(), 0.5);
            let b = gsk::PathBuilder::new();
            b.add_circle(&graphene::Point::new(cx - s * 1.3, cy + s * 0.3), s * 0.75);
            b.add_circle(&graphene::Point::new(cx, cy - s * 0.35), s * 0.85);
            b.add_circle(&graphene::Point::new(cx + s * 1.3, cy + s * 0.3), s * 0.75);
            snapshot.append_fill(&b.to_path(), gsk::FillRule::Winding, &fill);

            let jb = gsk::PathBuilder::new();
            jb.add_circle(&graphene::Point::new(cx, cy - s * 0.35), s * 0.22);
            snapshot.append_fill(&jb.to_path(), gsk::FillRule::Winding, ink);
        }

        fn draw_jack_mark(&self, snapshot: &gtk::Snapshot, cx: f32, cy: f32, w: f32, ink: &gtk::gdk::RGBA) {
            let s = w * 0.13;
            let b = gsk::PathBuilder::new();
            b.move_to(cx, cy - s);
            b.line_to(cx + s * 0.72, cy);
            b.line_to(cx, cy + s);
            b.line_to(cx - s * 0.72, cy);
            b.close();
            snapshot.append_stroke(&b.to_path(), &gsk::Stroke::new(w * 0.014), ink);
        }

        fn draw_pip(
            &self,
            obj: &super::CardWidget,
            snapshot: &gtk::Snapshot,
            suit: u8,
            w: f32,
            h: f32,
            px: f32,
            py: f32,
            flip: bool,
            ace: bool,
            ink: &gtk::gdk::RGBA,
        ) {
            let size_px = pip_font_px(self.size.get(), ace);
            let layout = obj.create_pango_layout(Some(suit_glyph(suit)));
            let mut desc = gtk::pango::FontDescription::from_string("Sans");
            desc.set_absolute_size(pango_px(size_px));
            layout.set_font_description(Some(&desc));
            let (pw, ph) = layout.pixel_size();
            snapshot.save();
            if flip {
                snapshot.translate(&graphene::Point::new(w * px / 100.0, h * py / 100.0));
                snapshot.rotate(180.0);
                snapshot.translate(&graphene::Point::new(-pw as f32 / 2.0, -ph as f32 / 2.0));
            } else {
                snapshot.translate(&graphene::Point::new(
                    w * px / 100.0 - pw as f32 / 2.0,
                    h * py / 100.0 - ph as f32 / 2.0,
                ));
            }
            snapshot.append_layout(&layout, ink);
            snapshot.restore();
        }
    }
}

glib::wrapper! {
    pub struct CardWidget(ObjectSubclass<imp::CardWidget>)
        @extends gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl CardWidget {
    pub fn pixel_size(size: CardSize) -> (i32, i32) {
        (card_width(size), card_height(size))
    }

    pub fn new_face(card: Card, face_letter: &str, size: CardSize) -> Self {
        let obj: CardWidget = glib::Object::new();
        obj.imp().kind.replace(CardKind::Face(card));
        obj.imp().size.set(size);
        obj.imp().face_letter.replace(face_letter.to_string());
        obj.imp().card_id.set(card.id);
        let (w, h) = Self::pixel_size(size);
        obj.set_size_request(w, h);
        obj.set_tooltip_text(Some(&label_for(card)));
        obj.add_css_class("card");
        obj.queue_draw();
        obj
    }

    pub fn new_back(size: CardSize, tint_red: bool) -> Self {
        let obj: CardWidget = glib::Object::new();
        obj.imp().kind.replace(CardKind::Back);
        obj.imp().size.set(size);
        obj.imp().tint_red.set(tint_red);
        let (w, h) = Self::pixel_size(size);
        obj.set_size_request(w, h);
        obj.add_css_class("card");
        obj.queue_draw();
        obj
    }

    pub fn set_selected(&self, v: bool) {
        self.imp().selected.set(v);
        self.queue_draw();
    }
    pub fn set_dimmed(&self, v: bool) {
        self.imp().dimmed.set(v);
        self.set_opacity(if v { 0.45 } else { 1.0 });
    }
    pub fn set_highlight(&self, v: bool) {
        self.imp().highlight.set(v);
        self.queue_draw();
    }
    pub fn card_id(&self) -> u8 {
        self.imp().card_id.get()
    }
}

fn label_for(card: Card) -> String {
    let suit = match card.suit() {
        CLUBS => "Clubs",
        DIAMONDS => "Diamonds",
        SPADES => "Spades",
        HEARTS => "Hearts",
        _ => "",
    };
    let rank = match card.rank() {
        11 => "Jack",
        12 => "Queen",
        13 => "King",
        14 => "Ace",
        r => return format!("{} {}", suit, r),
    };
    format!("{} of {}", rank, suit)
}

fn pip_positions(is_ace: bool, rank: u8) -> Vec<(f32, f32, bool)> {
    if is_ace {
        return vec![(50.0, 50.0, false)];
    }
    match rank {
        2 => vec![(50.0, 22.0, false), (50.0, 78.0, true)],
        3 => vec![(50.0, 20.0, false), (50.0, 50.0, false), (50.0, 80.0, true)],
        4 => vec![(32.0, 22.0, false), (68.0, 22.0, false), (32.0, 78.0, true), (68.0, 78.0, true)],
        5 => vec![
            (32.0, 22.0, false),
            (68.0, 22.0, false),
            (50.0, 50.0, false),
            (32.0, 78.0, true),
            (68.0, 78.0, true),
        ],
        6 => vec![
            (32.0, 22.0, false),
            (68.0, 22.0, false),
            (32.0, 50.0, false),
            (68.0, 50.0, false),
            (32.0, 78.0, true),
            (68.0, 78.0, true),
        ],
        7 => vec![
            (32.0, 20.0, false),
            (68.0, 20.0, false),
            (50.0, 36.0, false),
            (32.0, 50.0, false),
            (68.0, 50.0, false),
            (32.0, 80.0, true),
            (68.0, 80.0, true),
        ],
        8 => vec![
            (32.0, 18.0, false),
            (68.0, 18.0, false),
            (32.0, 40.0, false),
            (68.0, 40.0, false),
            (32.0, 60.0, true),
            (68.0, 60.0, true),
            (32.0, 82.0, true),
            (68.0, 82.0, true),
        ],
        9 => vec![
            (32.0, 18.0, false),
            (68.0, 18.0, false),
            (32.0, 38.0, false),
            (68.0, 38.0, false),
            (50.0, 50.0, false),
            (32.0, 62.0, true),
            (68.0, 62.0, true),
            (32.0, 82.0, true),
            (68.0, 82.0, true),
        ],
        10 => vec![
            (32.0, 16.0, false),
            (68.0, 16.0, false),
            (50.0, 28.0, false),
            (32.0, 38.0, false),
            (68.0, 38.0, false),
            (32.0, 62.0, true),
            (68.0, 62.0, true),
            (50.0, 72.0, true),
            (32.0, 84.0, true),
            (68.0, 84.0, true),
        ],
        _ => vec![],
    }
}
