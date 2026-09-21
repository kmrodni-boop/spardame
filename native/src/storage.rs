use crate::engine::*;
use serde::{Deserialize, Serialize};

const SAVE_VERSION: u32 = 1;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    #[serde(default)]
    pub variant: VariantId,
    #[serde(default)]
    pub difficulty: Difficulty,
    #[serde(default)]
    pub player_name: String,
    #[serde(default)]
    pub ai_names: [String; 3],
    #[serde(default)]
    pub muted: bool,
    #[serde(default)]
    pub locale: Locale,
    #[serde(default)]
    pub card_back: CardBackTint,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Stats {
    #[serde(default)]
    pub games_played: u32,
    #[serde(default)]
    pub games_won: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SaveBlob {
    #[serde(default)]
    version: u32,
    #[serde(default = "default_settings")]
    settings: Settings,
    #[serde(default)]
    stats: Stats,
    #[serde(default)]
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
    #[serde(default)]
    pub variant: VariantId,
    #[serde(default)]
    pub phase: Phase,
    #[serde(default)]
    pub hands: Vec<Vec<u8>>,
    #[serde(default)]
    pub taken: Vec<Vec<u8>>,
    #[serde(default)]
    pub trick: Vec<(usize, u8)>,
    #[serde(default)]
    pub turn: usize,
    #[serde(default)]
    pub hearts_broken: bool,
    #[serde(default)]
    pub trick_number: usize,
    #[serde(default)]
    pub hand_number: usize,
    #[serde(default)]
    pub scores: [i32; 4],
    #[serde(default)]
    pub hand_score: Option<SerializedHandScore>,
    #[serde(default)]
    pub pass_dir: PassDir,
    #[serde(default)]
    pub pass_selections: Vec<Option<Vec<u8>>>,
    #[serde(default)]
    pub received_pass: Vec<u8>,
    #[serde(default)]
    pub history: Vec<(Vec<(usize, u8)>, usize)>,
    #[serde(default)]
    pub winner: Option<usize>,
    #[serde(default)]
    pub tied: Vec<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializedHandScore {
    #[serde(default)]
    pub raw: [i32; 4],
    #[serde(default)]
    pub applied: [i32; 4],
    #[serde(default)]
    pub moon: Option<usize>,
    #[serde(default)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cards::Rng;

    // File-path resolution reads process-wide env vars (XDG_DATA_HOME), which
    // races under cargo's parallel test threads, so these tests exercise
    // (de)serialization directly instead of going through load_save/write_save.

    #[test]
    fn state_round_trips_through_serialization() {
        let mut rng = Rng::from_seed(7);
        let state = start_hand(VariantId::Spardame, 3, &[10, 20, 30, 40], &mut rng);
        let serialized = serialize_state(&state);
        let json = serde_json::to_string(&serialized).unwrap();
        let parsed: SerializedState = serde_json::from_str(&json).unwrap();
        let restored = deserialize_state(&parsed);
        assert_eq!(restored.variant, state.variant);
        assert_eq!(restored.phase, state.phase);
        assert_eq!(restored.hands, state.hands);
        assert_eq!(restored.turn, state.turn);
        assert_eq!(restored.scores, state.scores);
    }

    #[test]
    fn old_style_save_missing_new_fields_still_loads() {
        // Simulates a save written before some field existed: #[serde(default)]
        // must keep a future addition from wiping out the whole save.
        let minimal = r#"{
            "version": 1,
            "settings": { "player_name": "Kari" },
            "stats": {}
        }"#;
        let parsed: SaveBlob =
            serde_json::from_str(minimal).expect("must tolerate a save missing newer fields");
        assert_eq!(parsed.settings.player_name, "Kari");
        assert_eq!(parsed.settings.variant, VariantId::Spardame);
        assert_eq!(parsed.stats.games_played, 0);
        assert!(parsed.game.is_none());
    }

    #[test]
    fn corrupt_json_fails_to_parse_without_panicking() {
        let bad = "{not valid json";
        let result: Result<SaveBlob, _> = serde_json::from_str(bad);
        assert!(result.is_err());
    }
}
