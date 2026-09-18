use crate::engine::*;
use serde::{Deserialize, Serialize};

const SAVE_VERSION: u32 = 1;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub variant: VariantId,
    pub difficulty: Difficulty,
    pub player_name: String,
    pub ai_names: [String; 3],
    pub muted: bool,
    pub locale: Locale,
    pub card_back: CardBackTint,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Stats {
    pub games_played: u32,
    pub games_won: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SaveBlob {
    version: u32,
    settings: Settings,
    stats: Stats,
    game: Option<SerializedState>,
}

pub fn default_settings() -> Settings {
    Settings {
        variant: VariantId::Spardame,
        difficulty: Difficulty::Normal,
        player_name: String::new(),
        ai_names: ["Kari".into(), "Per".into(), "Liv".into()],
        muted: false,
        locale: Locale::En,
        card_back: CardBackTint::Red,
    }
}

pub fn default_stats() -> Stats {
    Stats::default()
}

// Serialization-friendly mirror of GameState.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializedState {
    pub variant: VariantId,
    pub phase: Phase,
    pub hands: Vec<Vec<u8>>,
    pub taken: Vec<Vec<u8>>,
    pub trick: Vec<(usize, u8)>,
    pub turn: usize,
    pub hearts_broken: bool,
    pub trick_number: usize,
    pub hand_number: usize,
    pub scores: [i32; 4],
    pub hand_score: Option<SerializedHandScore>,
    pub pass_dir: PassDir,
    pub pass_selections: Vec<Option<Vec<u8>>>,
    pub received_pass: Vec<u8>,
    pub history: Vec<(Vec<(usize, u8)>, usize)>,
    pub winner: Option<usize>,
    pub tied: Vec<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializedHandScore {
    pub raw: [i32; 4],
    pub applied: [i32; 4],
    pub moon: Option<usize>,
    pub jack_holder: Option<usize>,
}

use crate::cards::Card;

pub fn serialize_state(s: &GameState) -> SerializedState {
    SerializedState {
        variant: s.variant,
        phase: s.phase,
        hands: s.hands.iter().map(|h| h.iter().map(|c| c.id).collect()).collect(),
        taken: s.taken.iter().map(|h| h.iter().map(|c| c.id).collect()).collect(),
        trick: s.trick.iter().map(|p| (p.player, p.card.id)).collect(),
        turn: s.turn,
        hearts_broken: s.hearts_broken,
        trick_number: s.trick_number,
        hand_number: s.hand_number,
        scores: s.scores,
        hand_score: s.hand_score.as_ref().map(|h| SerializedHandScore {
            raw: h.raw,
            applied: h.applied,
            moon: h.moon,
            jack_holder: h.jack_holder,
        }),
        pass_dir: s.pass_dir,
        pass_selections: s
            .pass_selections
            .iter()
            .map(|opt| opt.as_ref().map(|v| v.iter().map(|c| c.id).collect()))
            .collect(),
        received_pass: s.received_pass.iter().map(|c| c.id).collect(),
        history: s
            .history
            .iter()
            .map(|t| (t.plays.iter().map(|p| (p.player, p.card.id)).collect(), t.winner))
            .collect(),
        winner: s.winner,
        tied: s.tied.clone(),
    }
}

pub fn deserialize_state(s: &SerializedState) -> GameState {
    let card_from_id = |id: u8| Card::new(id / 16, id % 16);
    GameState {
        variant: s.variant,
        phase: s.phase,
        hands: s
            .hands
            .iter()
            .map(|h| h.iter().map(|&id| card_from_id(id)).collect())
            .collect(),
        taken: s
            .taken
            .iter()
            .map(|h| h.iter().map(|&id| card_from_id(id)).collect())
            .collect(),
        trick: s
            .trick
            .iter()
            .map(|(p, id)| crate::engine::Play { player: *p, card: card_from_id(*id) })
            .collect(),
        turn: s.turn,
        hearts_broken: s.hearts_broken,
        trick_number: s.trick_number,
        hand_number: s.hand_number,
        scores: s.scores,
        hand_score: s.hand_score.as_ref().map(|h| crate::engine::HandScore {
            raw: h.raw,
            applied: h.applied,
            moon: h.moon,
            jack_holder: h.jack_holder,
        }),
        pass_dir: s.pass_dir,
        pass_selections: s
            .pass_selections
            .iter()
            .map(|opt| opt.as_ref().map(|v| v.iter().map(|&id| card_from_id(id)).collect()))
            .collect(),
        received_pass: s.received_pass.iter().map(|&id| card_from_id(id)).collect(),
        history: s
            .history
            .iter()
            .map(|(plays, w)| crate::engine::TrickRecord {
                plays: plays
                    .iter()
                    .map(|(p, id)| crate::engine::Play { player: *p, card: card_from_id(*id) })
                    .collect(),
                winner: *w,
            })
            .collect(),
        winner: s.winner,
        tied: s.tied.clone(),
    }
}

fn save_path() -> std::path::PathBuf {
    let base = std::env::var("XDG_DATA_HOME")
        .ok()
        .map(std::path::PathBuf::from)
        .or_else(|| {
            std::env::var("HOME").ok().map(|h| {
                let mut p = std::path::PathBuf::from(h);
                p.push(".local");
                p.push("share");
                p
            })
        })
        .unwrap_or_else(|| std::path::PathBuf::from("."));
    let mut p = base;
    p.push("spardame");
    let _ = std::fs::create_dir_all(&p);
    p.push("save.json");
    p
}

pub fn load_save() -> (Settings, Stats, Option<GameState>) {
    let path = save_path();
    let raw = match std::fs::read_to_string(&path) {
        Ok(s) => s,
        Err(_) => return (default_settings(), default_stats(), None),
    };
    let parsed: SaveBlob = match serde_json::from_str(&raw) {
        Ok(b) => b,
        Err(_) => return (default_settings(), default_stats(), None),
    };
    let mut settings = parsed.settings;
    if settings.player_name == "Du" && parsed.version == 1 && settings.locale == Locale::En {
        settings.player_name.clear();
    }
    if !matches!(settings.card_back, CardBackTint::Red | CardBackTint::Blue) {
        settings.card_back = CardBackTint::Red;
    }
    let game = parsed.game.as_ref().map(deserialize_state);
    (settings, parsed.stats, game)
}

pub fn write_save(settings: &Settings, stats: &Stats, game: Option<&GameState>) {
    let blob = SaveBlob {
        version: SAVE_VERSION,
        settings: settings.clone(),
        stats: stats.clone(),
        game: game.map(serialize_state),
    };
    if let Ok(json) = serde_json::to_string_pretty(&blob) {
        let _ = std::fs::write(save_path(), json);
    }
}
