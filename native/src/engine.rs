use crate::cards::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VariantId {
    Spardame,
    Hjerter,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Difficulty {
    Easy,
    Normal,
    Hard,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PassDir {
    Left,
    Right,
    Across,
    Hold,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Phase {
    Passing,
    Playing,
    TrickEnd,
    HandEnd,
    GameOver,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CardBackTint {
    Red,
    Blue,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Locale {
    En,
    Nb,
}

#[derive(Debug, Clone, Copy)]
pub struct Play {
    pub player: PlayerId,
    pub card: Card,
}

#[derive(Debug, Clone)]
pub struct TrickRecord {
    pub plays: Vec<Play>,
    pub winner: PlayerId,
}

#[derive(Debug, Clone)]
pub struct HandScore {
    pub raw: [i32; 4],
    pub applied: [i32; 4],
    pub moon: Option<PlayerId>,
    pub jack_holder: Option<PlayerId>,
}

#[derive(Debug, Clone)]
pub struct Variant {
    pub id: VariantId,
    pub queen_spades: i32,
    pub ace_hearts: i32,
    pub other_hearts: i32,
    pub jack_diamonds: i32,
    pub pass_count: usize,
    pub game_limit: i32,
    pub can_pass_queen: bool,
    pub moon_others: i32,
}

pub fn get_variant(id: VariantId) -> Variant {
    match id {
        VariantId::Spardame => Variant {
            id,
            queen_spades: 100,
            ace_hearts: 20,
            other_hearts: 10,
            jack_diamonds: -100,
            pass_count: 3,
            game_limit: 500,
            can_pass_queen: false,
            moon_others: 100,
        },
        VariantId::Hjerter => Variant {
            id,
            queen_spades: 13,
            ace_hearts: 1,
            other_hearts: 1,
            jack_diamonds: 0,
            pass_count: 3,
            game_limit: 100,
            can_pass_queen: true,
            moon_others: 26,
        },
    }
}

pub const PASS_CYCLE: [PassDir; 4] = [
    PassDir::Left,
    PassDir::Right,
    PassDir::Across,
    PassDir::Hold,
];

pub fn pass_offset(dir: PassDir) -> usize {
    match dir {
        PassDir::Left => 1,
        PassDir::Right => 3,
        PassDir::Across => 2,
        PassDir::Hold => 0,
    }
}

#[derive(Debug, Clone)]
pub struct GameState {
    pub variant: VariantId,
    pub phase: Phase,
    pub hands: Vec<Vec<Card>>,
    pub taken: Vec<Vec<Card>>,
    pub trick: Vec<Play>,
    pub turn: PlayerId,
    pub hearts_broken: bool,
    pub trick_number: usize,
    pub hand_number: usize,
    pub scores: [i32; 4],
    pub hand_score: Option<HandScore>,
    pub pass_dir: PassDir,
    pub pass_selections: Vec<Option<Vec<Card>>>,
    pub received_pass: Vec<Card>,
    pub history: Vec<TrickRecord>,
    pub winner: Option<PlayerId>,
    pub tied: Vec<PlayerId>,
}

pub fn card_points(card: Card, variant_id: VariantId) -> i32 {
    let v = get_variant(variant_id);
    if is_queen_of_spades(card) {
        return v.queen_spades;
    }
    if is_jack_of_diamonds(card) {
        return v.jack_diamonds;
    }
    if is_heart(card) {
        return if card.rank() == 14 { v.ace_hearts } else { v.other_hearts };
    }
    0
}

pub fn deal_hands(rng: &mut Rng) -> Vec<Vec<Card>> {
    let mut deck = make_deck();
    shuffle(&mut deck, rng);
    let mut hands: Vec<Vec<Card>> = vec![Vec::new(), Vec::new(), Vec::new(), Vec::new()];
    for (i, card) in deck.into_iter().enumerate() {
        hands[i % 4].push(card);
    }
    for h in &mut hands {
        sort_hand(h);
    }
    hands
}

pub fn holder_of(hands: &[Vec<Card>], target_suit: Suit, target_rank: Rank) -> PlayerId {
    for (p, hand) in hands.iter().enumerate() {
        if hand.iter().any(|c| c.suit() == target_suit && c.rank() == target_rank) {
            return p;
        }
    }
    0
}

pub fn next_player(p: PlayerId) -> PlayerId {
    (p + 1) % 4
}

pub fn empty_pass() -> Vec<Option<Vec<Card>>> {
    vec![None, None, None, None]
}

pub fn start_hand(
    variant: VariantId,
    hand_number: usize,
    scores: &[i32; 4],
    rng: &mut Rng,
) -> GameState {
    let hands = deal_hands(rng);
    let pass_dir = PASS_CYCLE[hand_number % 4];
    let lead = holder_of(&hands, CLUBS, 2);
    let passing = !matches!(pass_dir, PassDir::Hold);
    GameState {
        variant,
        phase: if passing { Phase::Passing } else { Phase::Playing },
        hands,
        taken: vec![Vec::new(), Vec::new(), Vec::new(), Vec::new()],
        trick: Vec::new(),
        turn: lead,
        hearts_broken: false,
        trick_number: 0,
        hand_number,
        scores: *scores,
        hand_score: None,
        pass_dir,
        pass_selections: empty_pass(),
        received_pass: Vec::new(),
        history: Vec::new(),
        winner: None,
        tied: Vec::new(),
    }
}

pub fn create_game(variant: VariantId, rng: &mut Rng) -> GameState {
    start_hand(variant, 0, &[0, 0, 0, 0], rng)
}

pub fn passable_cards(state: &GameState, player: PlayerId) -> Vec<Card> {
    let v = get_variant(state.variant);
    let hand = &state.hands[player];
    if v.can_pass_queen {
        return hand.clone();
    }
    hand.iter().filter(|c| !is_queen_of_spades(**c)).copied().collect()
}

pub fn set_pass_selection(
    state: &GameState,
    player: PlayerId,
    cards: &[Card],
) -> Result<GameState, String> {
    let v = get_variant(state.variant);
    if state.phase != Phase::Passing {
        return Err("Ikke i byttefase".into());
    }
    if cards.len() != v.pass_count {
        return Err(format!("Velg {} kort", v.pass_count));
    }
    let hand = &state.hands[player];
    for &card in cards {
        if !hand.iter().any(|c| c.id == card.id) {
            return Err("Kortet er ikke på hånden".into());
        }
        if !v.can_pass_queen && is_queen_of_spades(card) {
            return Err("Spar dame kan ikke byttes bort".into());
        }
    }
    let ids: Vec<u8> = cards.iter().map(|c| c.id).collect();
    let unique: std::collections::HashSet<u8> = ids.iter().copied().collect();
    if unique.len() != cards.len() {
        return Err("Duplikat i byttet".into());
    }
    let mut pass_selections = state.pass_selections.clone();
    pass_selections[player] = Some(cards.to_vec());
    let mut next = state.clone();
    next.pass_selections = pass_selections;
    Ok(next)
}

pub fn commit_pass(state: &GameState) -> Result<GameState, String> {
    let v = get_variant(state.variant);
    if state.phase != Phase::Passing {
        return Ok(state.clone());
    }
    if matches!(state.pass_dir, PassDir::Hold) {
        let mut next = state.clone();
        next.phase = Phase::Playing;
        next.turn = holder_of(&state.hands, CLUBS, 2);
        return Ok(next);
    }
    if state.pass_selections.iter().any(|s| s.as_ref().map_or(true, |c| c.len() != v.pass_count)) {
        return Err("Alle spillere må velge kort før byttet".into());
    }
    let offset = pass_offset(state.pass_dir);
    let mut next_hands: Vec<Vec<Card>> = state.hands.iter().map(|h| h.clone()).collect();
    let mut received: Vec<Vec<Card>> = vec![Vec::new(), Vec::new(), Vec::new(), Vec::new()];
    for p in 0..4 {
        let giving = state.pass_selections[p].clone().unwrap();
        let ids: Vec<u8> = giving.iter().map(|c| c.id).collect();
        next_hands[p] = without_cards(&next_hands[p], &ids);
        let dest = (p + offset) % 4;
        received[dest] = giving;
    }
    for p in 0..4 {
        let mut merged = next_hands[p].clone();
        merged.extend(received[p].iter().copied());
        sort_hand(&mut merged);
        next_hands[p] = merged;
    }
    let mut next = state.clone();
    next.hands = next_hands;
    next.received_pass = received[0].clone();
    next.pass_selections = empty_pass();
    next.phase = Phase::Playing;
    next.turn = holder_of(&next.hands, CLUBS, 2);
    Ok(next)
}

pub fn legal_moves(state: &GameState, player: PlayerId) -> Vec<Card> {
    if state.phase != Phase::Playing {
        return Vec::new();
    }
    if state.turn != player {
        return Vec::new();
    }
    let hand = &state.hands[player];
    if hand.is_empty() {
        return Vec::new();
    }
    if state.trick.is_empty() {
        if state.trick_number == 0 {
            if let Some(two) = hand.iter().copied().find(|c| is_two_of_clubs(*c)) {
                return vec![two];
            }
        }
        if !state.hearts_broken {
            let non_hearts: Vec<Card> = hand.iter().filter(|c| !is_heart(**c)).copied().collect();
            if !non_hearts.is_empty() {
                return non_hearts;
            }
        }
        return hand.clone();
    }
    let lead_suit = state.trick[0].card.suit();
    let matching: Vec<Card> = hand.iter().filter(|c| c.suit() == lead_suit).copied().collect();
    if !matching.is_empty() {
        return matching;
    }
    if state.trick_number == 0 {
        let safe: Vec<Card> = hand.iter().filter(|c| !is_penalty(**c)).copied().collect();
        if !safe.is_empty() {
            return safe;
        }
    }
    hand.clone()
}

pub fn is_legal_play(state: &GameState, player: PlayerId, card_id: u8) -> bool {
    legal_moves(state, player).iter().any(|c| c.id == card_id)
}

pub fn trick_winner(plays: &[Play]) -> PlayerId {
    let lead_suit = plays[0].card.suit();
    let mut best = plays[0];
    for play in plays.iter().skip(1) {
        if play.card.suit() == lead_suit && play.card.rank() > best.card.rank() {
            best = *play;
        }
    }
    best.player
}

pub fn play_card(state: &GameState, player: PlayerId, card_id: u8) -> Result<GameState, String> {
    if !is_legal_play(state, player, card_id) {
        return Err("Ulovlig trekk".into());
    }
    let card = match find_card(&state.hands[player], card_id) {
        Some(c) => c,
        None => return Err("Kortet er ikke på hånden".into()),
    };
    let mut hands: Vec<Vec<Card>> = state
        .hands
        .iter()
        .enumerate()
        .map(|(i, h)| if i == player { h.iter().filter(|c| c.id != card_id).copied().collect() } else { h.clone() })
        .collect();
    let _ = &mut hands;
    let mut trick = state.trick.clone();
    trick.push(Play { player, card });
    let broken = state.hearts_broken || is_heart(card);
    let mut next = state.clone();
    if trick.len() < 4 {
        next.hands = hands;
        next.trick = trick;
        next.turn = next_player(player);
        next.hearts_broken = broken;
        return Ok(next);
    }
    next.hands = hands;
    next.trick = trick;
    next.hearts_broken = broken;
    next.phase = Phase::TrickEnd;
    Ok(next)
}

pub fn taken_points(taken: &[Card], variant_id: VariantId) -> i32 {
    taken.iter().map(|c| card_points(*c, variant_id)).sum()
}

pub fn has_shot_the_moon(taken: &[Card]) -> bool {
    let hearts = taken.iter().filter(|c| is_heart(**c)).count();
    let queen = taken.iter().any(|c| is_queen_of_spades(*c));
    hearts == 13 && queen
}

fn score_from_taken(taken: &[Vec<Card>], variant_id: VariantId) -> HandScore {
    let raw = [
        taken_points(&taken[0], variant_id),
        taken_points(&taken[1], variant_id),
        taken_points(&taken[2], variant_id),
        taken_points(&taken[3], variant_id),
    ];
    let moon = (0..4).find(|&p| has_shot_the_moon(&taken[p]));
    let jack_holder = (0..4).find(|&p| taken[p].iter().any(|c| is_jack_of_diamonds(*c)));
    let mut applied = raw;
    if let Some(m) = moon {
        let v = get_variant(variant_id);
        for p in 0..4 {
            if p == m {
                let jack = if v.jack_diamonds != 0 && jack_holder == Some(m) { v.jack_diamonds } else { 0 };
                applied[p] = jack;
            } else {
                applied[p] = v.moon_others;
            }
        }
    }
    HandScore { raw, applied, moon, jack_holder }
}

pub fn resolve_trick(state: &GameState) -> GameState {
    if state.phase != Phase::TrickEnd || state.trick.len() != 4 {
        return state.clone();
    }
    let winner = trick_winner(&state.trick);
    let mut taken = state.taken.clone();
    let won_cards: Vec<Card> = state.trick.iter().map(|p| p.card).collect();
    taken[winner].extend(won_cards);
    let history = {
        let mut h = state.history.clone();
        h.push(TrickRecord { plays: state.trick.clone(), winner });
        h
    };
    let last_trick = state.trick_number >= 12;
    let mut next = state.clone();
    next.taken = taken;
    next.history = history;
    next.trick = Vec::new();
    next.trick_number = state.trick_number + 1;
    if !last_trick {
        next.phase = Phase::Playing;
        next.turn = winner;
        return next;
    }
    let hand_score = score_from_taken(&next.taken, state.variant);
    let mut scores = state.scores;
    for i in 0..4 {
        scores[i] += hand_score.applied[i];
    }
    next.scores = scores;
    next.hand_score = Some(hand_score.clone());
    let v = get_variant(state.variant);
    let over = scores.iter().any(|&s| s >= v.game_limit);
    if !over {
        next.phase = Phase::HandEnd;
        return next;
    }
    let min = *scores.iter().min().unwrap();
    let tied: Vec<PlayerId> = (0..4).filter(|&p| scores[p] == min).collect();
    next.phase = Phase::GameOver;
    next.winner = Some(tied[0]);
    next.tied = tied.clone();
    next
}

pub fn continue_game(state: &GameState, rng: &mut Rng) -> GameState {
    if state.phase != Phase::HandEnd {
        return state.clone();
    }
    start_hand(state.variant, state.hand_number + 1, &state.scores, rng)
}

pub fn current_winner_of_trick(trick: &[Play]) -> Option<PlayerId> {
    if trick.is_empty() {
        None
    } else {
        Some(trick_winner(trick))
    }
}

pub fn suit_led(state: &GameState) -> Option<Suit> {
    state.trick.first().map(|p| p.card.suit())
}

pub fn queen_still_out(state: &GameState) -> bool {
    if state.trick.iter().any(|p| is_queen_of_spades(p.card)) {
        return false;
    }
    !state.taken.iter().any(|pile| pile.iter().any(|c| is_queen_of_spades(*c)))
}

pub fn jack_still_out(state: &GameState) -> bool {
    if state.trick.iter().any(|p| is_jack_of_diamonds(p.card)) {
        return false;
    }
    !state.taken.iter().any(|pile| pile.iter().any(|c| is_jack_of_diamonds(*c)))
}

pub fn point_cards_taken(pile: &[Card], variant_id: VariantId) -> Vec<Card> {
    pile.iter().filter(|c| card_points(**c, variant_id) != 0).copied().collect()
}

pub fn remaining_cards(state: &GameState, viewer: PlayerId) -> Vec<Card> {
    let mut seen = std::collections::HashSet::new();
    for c in &state.hands[viewer] {
        seen.insert(c.id);
    }
    for pile in &state.taken {
        for c in pile {
            seen.insert(c.id);
        }
    }
    for p in &state.trick {
        seen.insert(p.card.id);
    }
    make_deck().into_iter().filter(|c| !seen.contains(&c.id)).collect()
}

pub fn pass_recipient(from: PlayerId, dir: PassDir) -> PlayerId {
    (from + pass_offset(dir)) % 4
}
