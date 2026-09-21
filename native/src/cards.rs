use serde::{Deserialize, Serialize};

pub type Suit = u8; // 0 clubs, 1 diamonds, 2 spades, 3 hearts
pub type Rank = u8; // 2..=14
pub type PlayerId = usize; // 0..3

pub const CLUBS: Suit = 0;
pub const DIAMONDS: Suit = 1;
pub const SPADES: Suit = 2;
pub const HEARTS: Suit = 3;

pub const SUITS: [Suit; 4] = [CLUBS, DIAMONDS, SPADES, HEARTS];
pub const RANKS: [Rank; 13] = [2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
pub const SUIT_ORDER: [u8; 4] = [0, 1, 2, 3];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Card {
    pub id: u8, // packed: suit * 16 + rank
}

impl Card {
    pub fn new(suit: Suit, rank: Rank) -> Self {
        Self { id: suit * 16 + rank }
    }
    pub fn suit(self) -> Suit {
        self.id / 16
    }
    pub fn rank(self) -> Rank {
        self.id % 16
    }
}

pub fn rank_code(rank: Rank) -> &'static str {
    match rank {
        11 => "J",
        12 => "Q",
        13 => "K",
        14 => "A",
        r => match r {
            2 => "2",
            3 => "3",
            4 => "4",
            5 => "5",
            6 => "6",
            7 => "7",
            8 => "8",
            9 => "9",
            10 => "10",
            _ => "",
        },
    }
}

pub fn make_deck() -> Vec<Card> {
    let mut deck = Vec::with_capacity(52);
    for &suit in &SUITS {
        for &rank in &RANKS {
            deck.push(Card::new(suit, rank));
        }
    }
    deck
}

pub struct Rng {
    state: u32,
}

impl Default for Rng {
    fn default() -> Self {
        Self::new()
    }
}

impl Rng {
    pub fn new() -> Self {
        let seed = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos() as u32)
            .unwrap_or(0xdead_beef)
            .wrapping_mul(2654435761);
        Self::from_seed(seed | 1)
    }
    pub fn from_seed(seed: u32) -> Self {
        Self { state: seed }
    }
    pub fn next_f64(&mut self) -> f64 {
        self.state = self.state.wrapping_mul(1664525).wrapping_add(1013904223);
        (self.state as f64) / 4_294_967_296.0
    }
}

pub fn shuffle(items: &mut Vec<Card>, rng: &mut Rng) {
    let n = items.len();
    for i in (1..n).rev() {
        let j = (rng.next_f64() * (i as f64 + 1.0)) as usize;
        items.swap(i, j);
    }
}

pub fn sort_hand(hand: &mut Vec<Card>) {
    hand.sort_by(|a, b| {
        let sa = SUIT_ORDER[a.suit() as usize].cmp(&SUIT_ORDER[b.suit() as usize]);
        sa.then(a.rank().cmp(&b.rank()))
    });
}

pub fn is_queen_of_spades(c: Card) -> bool {
    c.suit() == SPADES && c.rank() == 12
}
pub fn is_jack_of_diamonds(c: Card) -> bool {
    c.suit() == DIAMONDS && c.rank() == 11
}
pub fn is_two_of_clubs(c: Card) -> bool {
    c.suit() == CLUBS && c.rank() == 2
}
pub fn is_heart(c: Card) -> bool {
    c.suit() == HEARTS
}
pub fn is_penalty(c: Card) -> bool {
    is_heart(c) || is_queen_of_spades(c)
}

pub fn find_card(hand: &[Card], id: u8) -> Option<Card> {
    hand.iter().find(|c| c.id == id).copied()
}

pub fn without_cards(hand: &[Card], ids: &[u8]) -> Vec<Card> {
    hand.iter().filter(|c| !ids.contains(&c.id)).copied().collect()
}
