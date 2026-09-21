use crate::cards::*;
use crate::engine::*;

pub fn think_delay_ms(difficulty: Difficulty, legal_count: usize) -> u32 {
    let base = match difficulty {
        Difficulty::Easy => 520,
        Difficulty::Normal => 680,
        Difficulty::Hard => 860,
    };
    let extra = if legal_count > 6 { 180 } else { 90 };
    base + (rand_f64() * extra as f64) as u32
}

fn rand_f64() -> f64 {
    use std::cell::Cell;
    thread_local! {
        static SEED: Cell<u32> = const { Cell::new(0x1234_5678) };
    }
    SEED.with(|s| {
        let mut v = s.get().wrapping_mul(1664525).wrapping_add(1013904223);
        v ^= std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.subsec_nanos())
            .unwrap_or(0);
        s.set(v);
        (v >> 8) as f64 / 16777216.0
    })
}

fn noise(difficulty: Difficulty) -> f64 {
    (rand_f64() - 0.5)
        * match difficulty {
            Difficulty::Easy => 80.0,
            Difficulty::Normal => 12.0,
            Difficulty::Hard => 2.0,
        }
}

fn suit_count(hand: &[Card], suit: Suit) -> usize {
    hand.iter().filter(|c| c.suit() == suit).count()
}

pub fn choose_pass_cards(state: &GameState, player: PlayerId, difficulty: Difficulty) -> Vec<Card> {
    let v = get_variant(state.variant);
    let hand = &state.hands[player];
    let pool: Vec<Card> = if v.can_pass_queen {
        hand.clone()
    } else {
        hand.iter().filter(|c| !is_queen_of_spades(**c)).copied().collect()
    };
    let need = v.pass_count;

    let mut scored: Vec<(Card, f64)> = pool
        .iter()
        .map(|&card| (card, pass_score(card, hand, v.can_pass_queen) + noise(difficulty)))
        .collect();
    scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

    let mut chosen: Vec<Card> = Vec::new();
    let mut used = std::collections::HashSet::new();
    let try_void = best_void_candidates(&pool, hand);
    for card in try_void {
        if chosen.len() >= need {
            break;
        }
        if used.contains(&card.id) {
            continue;
        }
        if !v.can_pass_queen && is_queen_of_spades(card) {
            continue;
        }
        chosen.push(card);
        used.insert(card.id);
    }
    for (card, _) in &scored {
        if chosen.len() >= need {
            break;
        }
        if used.contains(&card.id) {
            continue;
        }
        chosen.push(*card);
        used.insert(card.id);
    }
    chosen.truncate(need);
    chosen
}

fn pass_score(card: Card, hand: &[Card], can_pass_queen: bool) -> f64 {
    if is_jack_of_diamonds(card) {
        return -80.0;
    }
    if is_queen_of_spades(card) {
        return if can_pass_queen && suit_count(hand, SPADES) <= 4 { 100.0 } else { -20.0 };
    }
    if card.suit() == HEARTS && card.rank() >= 12 {
        return 70.0 + card.rank() as f64;
    }
    if card.suit() == SPADES && card.rank() >= 13 {
        return if suit_count(hand, SPADES) <= 3 { 90.0 } else { 40.0 };
    }
    if card.rank() >= 13 {
        return 35.0 + card.rank() as f64;
    }
    if card.rank() == 12 {
        return 20.0;
    }
    card.rank() as f64 * 0.3
}

fn best_void_candidates(pool: &[Card], hand: &[Card]) -> Vec<Card> {
    let suits = [CLUBS, DIAMONDS, HEARTS];
    let mut best: Vec<Card> = Vec::new();
    for &suit in &suits {
        let cards: Vec<Card> = pool.iter().filter(|c| c.suit() == suit).copied().collect();
        let total = suit_count(hand, suit);
        if total > 0 && total <= 3 && cards.len() == total
            && (best.is_empty() || cards.len() < best.len()) {
                best = cards;
            }
    }
    best
}

fn moon_urge(state: &GameState, player: PlayerId) -> u8 {
    let taken = &state.taken[player];
    let hearts = taken.iter().filter(|c| is_heart(**c)).count();
    let has_q = taken.iter().any(|c| is_queen_of_spades(*c));
    let points_held = hearts + if has_q { 5 } else { 0 };
    if points_held < 6 {
        return 0;
    }
    let hand = &state.hands[player];
    let high_hearts = hand.iter().filter(|c| c.suit() == HEARTS && c.rank() >= 12).count();
    let remaining_hearts = 13 - hearts
        - state
            .trick
            .iter()
            .filter(|p| is_heart(p.card))
            .count();
    if has_q && hearts >= 8 {
        return 2;
    }
    if points_held >= 9 && high_hearts >= 1 && remaining_hearts <= 6 {
        return 1;
    }
    0
}

pub fn choose_play(state: &GameState, player: PlayerId, difficulty: Difficulty) -> Card {
    let legal = legal_moves(state, player);
    if legal.is_empty() {
        if let Some(&card) = state.hands.get(player).and_then(|h| h.first()) {
            return card;
        }
        return Card::new(CLUBS, 2);
    }
    if legal.len() == 1 {
        return legal[0];
    }
    if matches!(difficulty, Difficulty::Easy) && rand_f64() < 0.45 {
        let idx = (rand_f64() * legal.len() as f64) as usize;
        return legal[idx.min(legal.len() - 1)];
    }
    let moon = moon_urge(state, player);
    let mut scored: Vec<(Card, f64)> = legal
        .iter()
        .map(|&card| (card, evaluate_play(state, player, card, moon, difficulty) + noise(difficulty)))
        .collect();
    scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    scored[0].0
}

fn evaluate_play(
    state: &GameState,
    player: PlayerId,
    card: Card,
    moon: u8,
    difficulty: Difficulty,
) -> f64 {
    let trick = &state.trick;
    let v = get_variant(state.variant);
    let queen_out = queen_still_out(state);
    let jack_out = v.jack_diamonds != 0 && jack_still_out(state);
    let unseen: Vec<Card> = if matches!(difficulty, Difficulty::Hard) {
        remaining_cards(state, player)
    } else {
        Vec::new()
    };
    if trick.is_empty() {
        return eval_lead(state, player, card, moon, queen_out, jack_out, &unseen, difficulty);
    }
    let lead_suit = trick[0].card.suit();
    let following = card.suit() == lead_suit;
    let current = current_winner_of_trick(trick);
    let would_win = would_take(trick, card, player);
    let points_on_trick: i32 = trick
        .iter()
        .map(|p| if is_penalty_card_value(p.card) { 1 } else { 0 })
        .sum();
    let last_to_play = trick.len() == 3;
    if moon > 0 {
        if would_win {
            return 50.0 + card.rank() as f64 + if is_heart(card) { 8.0 } else { 0.0 } + if is_queen_of_spades(card) { 20.0 } else { 0.0 };
        }
        return 10.0 - card.rank() as f64;
    }
    if following {
        if lead_suit == SPADES {
            if is_queen_of_spades(card) {
                let max_spade: Rank = trick
                    .iter()
                    .filter(|p| p.card.suit() == SPADES)
                    .map(|p| p.card.rank())
                    .max()
                    .unwrap_or(0);
                if max_spade > 12 {
                    return 95.0;
                }
                if last_to_play && current != Some(player) {
                    return 40.0;
                }
                return -90.0;
            }
            if would_win && queen_out && card.rank() >= 13 {
                return -70.0;
            }
            if !would_win {
                return 30.0 + card.rank() as f64;
            }
            return 5.0 - card.rank() as f64;
        }
        if lead_suit == DIAMONDS && v.jack_diamonds != 0 {
            if is_jack_of_diamonds(card) {
                return if would_win { 80.0 } else { -60.0 };
            }
            if would_win && jack_out && card.rank() > 11 {
                return 55.0;
            }
            if would_win && !jack_out {
                return 8.0 - card.rank() as f64;
            }
        }
        if lead_suit == HEARTS {
            return if !would_win { 40.0 + card.rank() as f64 } else { -20.0 - card.rank() as f64 };
        }
        if !would_win {
            return 25.0 + card.rank() as f64 * 0.4;
        }
        if points_on_trick > 0 {
            return -30.0 - card.rank() as f64;
        }
        if last_to_play {
            return 8.0 - card.rank() as f64 * 0.2;
        }
        return 5.0 - card.rank() as f64 * 0.5;
    }
    // Void: dumping
    if is_queen_of_spades(card) {
        return 120.0;
    }
    if is_jack_of_diamonds(card) {
        return -100.0;
    }
    if is_heart(card) && card.rank() >= 12 {
        return 70.0 + card.rank() as f64;
    }
    if is_heart(card) {
        return 40.0 + card.rank() as f64;
    }
    if card.suit() == SPADES && card.rank() >= 13 && queen_out {
        return 85.0;
    }
    if card.rank() >= 13 {
        return 45.0 + card.rank() as f64;
    }
    if card.rank() >= 11 {
        return 20.0 + card.rank() as f64;
    }
    card.rank() as f64
}

#[allow(clippy::too_many_arguments)]
fn eval_lead(
    state: &GameState,
    player: PlayerId,
    card: Card,
    moon: u8,
    queen_out: bool,
    jack_out: bool,
    unseen: &[Card],
    _difficulty: Difficulty,
) -> f64 {
    let v = get_variant(state.variant);
    let hand = &state.hands[player];
    if moon > 0 {
        if is_heart(card) || is_queen_of_spades(card) {
            return 60.0 + card.rank() as f64;
        }
        return 20.0 + card.rank() as f64;
    }
    if is_queen_of_spades(card) {
        return -100.0;
    }
    if is_heart(card) {
        return -40.0 - card.rank() as f64;
    }
    if card.suit() == SPADES && queen_out && card.rank() >= 13 {
        return -80.0;
    }
    if card.suit() == SPADES && queen_out {
        let my_spades: Vec<Card> = hand.iter().filter(|c| c.suit() == SPADES && c.rank() < 12).copied().collect();
        if my_spades.len() >= 3 && card.rank() <= 6 {
            return 18.0;
        }
        return -15.0;
    }
    if is_jack_of_diamonds(card) {
        return if jack_out { 10.0 } else { -20.0 };
    }
    let count = suit_count(hand, card.suit());
    let mut score = 20.0 - card.rank() as f64 + if count >= 4 { 6.0 } else { 0.0 };
    if card.suit() == CLUBS || card.suit() == DIAMONDS {
        score += 8.0;
    }
    if !unseen.is_empty() && difficulty_safe(unseen, card) {
        score += 4.0;
    }
    if v.jack_diamonds != 0 && card.suit() == DIAMONDS && card.rank() < 11 && jack_out {
        score += 12.0;
    }
    score
}

fn difficulty_safe(unseen: &[Card], card: Card) -> bool {
    let higher = unseen.iter().filter(|c| c.suit() == card.suit() && c.rank() > card.rank()).count();
    higher >= 2
}

fn would_take(trick: &[Play], card: Card, player: PlayerId) -> bool {
    let mut plays = trick.to_vec();
    plays.push(Play { player, card });
    let lead_suit = plays[0].card.suit();
    let mut best = plays[0];
    for play in plays.iter().skip(1) {
        if play.card.suit() == lead_suit && play.card.rank() > best.card.rank() {
            best = *play;
        }
    }
    best.player == player
}

fn is_penalty_card_value(card: Card) -> bool {
    is_heart(card) || is_queen_of_spades(card)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cards::Rng;

    const DIFFICULTIES: [Difficulty; 3] = [Difficulty::Easy, Difficulty::Normal, Difficulty::Hard];

    #[test]
    fn choose_pass_cards_returns_a_valid_selection() {
        for seed in 0..20u32 {
            let mut rng = Rng::from_seed(seed);
            // hand_number 0 -> PassDir::Left -> Phase::Passing
            let state = start_hand(VariantId::Spardame, 0, &[0, 0, 0, 0], &mut rng);
            let v = get_variant(state.variant);
            for player in 0..4 {
                for &difficulty in &DIFFICULTIES {
                    let chosen = choose_pass_cards(&state, player, difficulty);
                    assert_eq!(chosen.len(), v.pass_count);
                    let ids: std::collections::HashSet<u8> = chosen.iter().map(|c| c.id).collect();
                    assert_eq!(ids.len(), v.pass_count, "no duplicate cards in the pass");
                    for card in &chosen {
                        assert!(
                            state.hands[player].iter().any(|c| c.id == card.id),
                            "AI tried to pass a card it doesn't hold"
                        );
                        assert!(
                            v.can_pass_queen || !is_queen_of_spades(*card),
                            "Spardame's queen of spades can't be passed"
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn choose_play_always_returns_a_legal_move() {
        for seed in 0..20u32 {
            for variant in [VariantId::Spardame, VariantId::Hjerter] {
                let mut rng = Rng::from_seed(seed);
                // hand_number 3 -> PassDir::Hold -> straight to Phase::Playing
                let mut state = start_hand(variant, 3, &[0, 0, 0, 0], &mut rng);
                let mut guard = 0;
                while state.phase == Phase::Playing && guard < 60 {
                    guard += 1;
                    let player = state.turn;
                    let legal = legal_moves(&state, player);
                    for &difficulty in &DIFFICULTIES {
                        let played = choose_play(&state, player, difficulty);
                        assert!(
                            legal.iter().any(|c| c.id == played.id),
                            "AI chose a card not among the legal moves"
                        );
                    }
                    let card = choose_play(&state, player, Difficulty::Normal);
                    state = play_card(&state, player, card.id).expect("AI move must be accepted");
                    if state.phase == Phase::TrickEnd {
                        state = resolve_trick(&state);
                    }
                }
                assert_ne!(state.phase, Phase::Playing, "hand never finished within 60 plays");
            }
        }
    }
}
