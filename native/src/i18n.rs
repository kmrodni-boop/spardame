use crate::cards::*;
use crate::engine::*;

pub struct Copy {
    pub lang_name: &'static str,
    pub card_game: &'static str,
    pub app_name: &'static str,
    pub tagline: &'static str,
    pub your_name: &'static str,
    pub you: &'static str,
    pub opponents: &'static str,
    pub easy: &'static str,
    pub normal: &'static str,
    pub hard: &'static str,
    pub new_game: &'static str,
    pub continue_: &'static str,
    pub rules: &'static str,
    pub sound_on: &'static str,
    pub sound_off: &'static str,
    pub card_back: &'static str,
    pub card_back_tint_red: &'static str,
    pub card_back_tint_blue: &'static str,
    pub to_menu: &'static str,
    pub close: &'static str,
    pub language: &'static str,
    pub passing: &'static str,
    pub lead: &'static str,
    pub hearts_broken: &'static str,
    pub queen_out: &'static str,
    pub jack_out: &'static str,
    pub trick: &'static str,
    pub hand_over: &'static str,
    pub game_over: &'static str,
    pub your_lead_two: &'static str,
    pub your_lead: &'static str,
    pub playing: &'static str,
    pub queen_of_spades: &'static str,
    pub jack_of_diamonds: &'static str,
    pub score_hand: &'static str,
    pub you_won: &'static str,
    pub draw: &'static str,
    pub scores: &'static str,
    pub player: &'static str,
    pub round: &'static str,
    pub total: &'static str,
    pub out: &'static str,
    pub menu: &'static str,
    pub quit: &'static str,
    pub next_hand: &'static str,
    pub rules_title: &'static str,
    pub rules_spardame_title: &'static str,
    pub rules_spardame_body: &'static str,
    pub rules_spardame_points: [&'static str; 4],
    pub rules_spardame_extra: &'static str,
    pub rules_hearts_title: &'static str,
    pub rules_hearts_body: &'static str,
    pub rules_play_title: &'static str,
    pub rules_play: [&'static str; 6],
    pub pass_left: &'static str,
    pub pass_right: &'static str,
    pub pass_across: &'static str,
    pub pass_hold: &'static str,
    pub cannot_pass_queen: &'static str,
    pub illegal: &'static str,
    pub pass_failed: &'static str,
    // dynamic fragments
    to: &'static str,
    games_won_pfx: &'static str,
    of: &'static str,
    send_pfx: &'static str,
    cards: &'static str,
    pick_pfx: &'static str,
    got_pfx: &'static str,
    you_take: &'static str,
    takes_trick: &'static str,
    thinking: &'static str,
    follow_pfx: &'static str,
    void_pfx: &'static str,
    won_sfx: &'static str,
    moon_sfx: &'static str,
    score_hint_pfx: &'static str,
    score_hint_sfx: &'static str,
    s_clubs: &'static str,
    s_diamonds: &'static str,
    s_spades: &'static str,
    s_hearts: &'static str,
    s_clubs_c: &'static str,
    s_diamonds_c: &'static str,
    s_spades_c: &'static str,
    s_hearts_c: &'static str,
    face_jack: &'static str,
    face_queen: &'static str,
    face_king: &'static str,
    face_ace: &'static str,
    var_spardame_name: &'static str,
    var_spardame_short: &'static str,
    var_spardame_blurb: &'static str,
    var_hjerter_name: &'static str,
    var_hjerter_short: &'static str,
    var_hjerter_blurb: &'static str,
}

impl Copy {
    pub fn to_limit(&self, n: i32) -> String {
        format!("{} {}", self.to, n)
    }
    pub fn games_won(&self, won: u32, played: u32) -> String {
        format!("{}: {} {} {}", self.games_won_pfx, won, self.of, played)
    }
    pub fn send_cards(&self, n: usize, of: usize, dir: &str) -> String {
        format!("{} {}/{} {} {}", self.send_pfx, n, of, self.cards, dir)
    }
    pub fn pick_cards(&self, n: usize, dir: &str) -> String {
        format!("{} {} {} {}.", self.pick_pfx, n, self.cards, dir)
    }
    pub fn pick_cards_short(&self, n: usize) -> String {
        format!("{} {} {}.", self.pick_pfx, n, self.cards)
    }
    pub fn got_cards(&self, n: usize, from: &str) -> String {
        format!("{} {} {} {}.", self.got_pfx, n, self.cards, from)
    }
    pub fn takes_trick(&self, name: &str, is_you: bool) -> String {
        if is_you {
            self.you_take.into()
        } else {
            format!("{} {}.", name, self.takes_trick)
        }
    }
    pub fn thinking(&self, name: &str) -> String {
        format!("{} {}{}", name, self.thinking, '…')
    }
    pub fn follow_suit(&self, suit: &str) -> String {
        format!("{} {}", self.follow_pfx, suit)
    }
    pub fn void_in(&self, suit: &str) -> String {
        format!("{} {}", self.void_pfx, suit)
    }
    pub fn won(&self, name: &str) -> String {
        format!("{} {}", name, self.won_sfx)
    }
    pub fn moon(&self, name: &str) -> String {
        format!("{} {}", name, self.moon_sfx)
    }
    pub fn score_hint(&self, limit: i32) -> String {
        format!("{} {} {}", self.score_hint_pfx, limit, self.score_hint_sfx)
    }
    pub fn suit_name(&self, suit: Suit) -> &'static str {
        match suit {
            CLUBS => self.s_clubs,
            DIAMONDS => self.s_diamonds,
            SPADES => self.s_spades,
            HEARTS => self.s_hearts,
            _ => "",
        }
    }
    pub fn suit_cap(&self, suit: Suit) -> &'static str {
        match suit {
            CLUBS => self.s_clubs_c,
            DIAMONDS => self.s_diamonds_c,
            SPADES => self.s_spades_c,
            HEARTS => self.s_hearts_c,
            _ => "",
        }
    }
    pub fn rank_face(&self, rank: Rank) -> &'static str {
        match rank {
            11 => self.face_jack,
            12 => self.face_queen,
            13 => self.face_king,
            14 => self.face_ace,
            _ => "",
        }
    }
    pub fn variant_name(&self, id: VariantId) -> &'static str {
        match id {
            VariantId::Spardame => self.var_spardame_name,
            VariantId::Hjerter => self.var_hjerter_name,
        }
    }
    pub fn variant_short(&self, id: VariantId) -> &'static str {
        match id {
            VariantId::Spardame => self.var_spardame_short,
            VariantId::Hjerter => self.var_hjerter_short,
        }
    }
    pub fn variant_blurb(&self, id: VariantId) -> &'static str {
        match id {
            VariantId::Spardame => self.var_spardame_blurb,
            VariantId::Hjerter => self.var_hjerter_blurb,
        }
    }
    pub fn pass_dir(&self, dir: PassDir) -> &'static str {
        match dir {
            PassDir::Left => self.pass_left,
            PassDir::Right => self.pass_right,
            PassDir::Across => self.pass_across,
            PassDir::Hold => self.pass_hold,
        }
    }
    pub fn card_label(&self, card: Card) -> String {
        if is_queen_of_spades(card) {
            return self.queen_of_spades.into();
        }
        if is_jack_of_diamonds(card) {
            return self.jack_of_diamonds.into();
        }
        let rank = match card.rank() {
            14 => self.face_ace,
            13 => self.face_king,
            12 => self.face_queen,
            11 => self.face_jack,
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
        };
        format!("{} {}", self.suit_cap(card.suit()), rank)
    }
    pub fn face_letter(&self, card: Card) -> &'static str {
        match card.rank() {
            11 => self.face_jack,
            12 => self.face_queen,
            13 => self.face_king,
            14 => self.face_ace,
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
}

pub fn strings(locale: Locale) -> Copy {
    match locale {
        Locale::En => EN,
        Locale::Nb => NB,
    }
}

#[allow(dead_code)]
pub fn html_lang(locale: Locale) -> &'static str {
    match locale {
        Locale::En => "en",
        Locale::Nb => "nb",
    }
}

const EN: Copy = Copy {
    lang_name: "English",
    card_game: "Card game",
    app_name: "Hearts",
    tagline:
        "Avoid hearts and the queen of spades. Take the jack of diamonds. Four players, three opponents, official rules.",
    your_name: "Your name",
    you: "You",
    opponents: "Opponents",
    easy: "Easy",
    normal: "Medium",
    hard: "Hard",
    new_game: "New game",
    continue_: "Continue",
    rules: "Rules",
    sound_on: "Sound on",
    sound_off: "Sound off",
    card_back: "Card back",
    card_back_tint_red: "Red",
    card_back_tint_blue: "Blue",
    to_menu: "Back to menu",
    close: "Close",
    language: "Language",
    passing: "Passing",
    lead: "Lead",
    hearts_broken: "Hearts broken",
    queen_out: "Queen out",
    jack_out: "Jack out",
    trick: "Trick",
    hand_over: "Hand complete.",
    game_over: "Game over.",
    your_lead_two: "Your lead — play the 2 of clubs.",
    your_lead: "Your lead — play a card.",
    playing: "Playing",
    queen_of_spades: "Queen of spades",
    jack_of_diamonds: "Jack of diamonds",
    score_hand: "Hand complete",
    you_won: "You won",
    draw: "Draw",
    scores: "Scores",
    player: "Player",
    round: "Hand",
    total: "Total",
    out: "out",
    menu: "Menu",
    quit: "Quit",
    next_hand: "Next hand",
    rules_title: "Rules",
    rules_spardame_title: "Queen of Spades",
    rules_spardame_body:
        "Four players, 52 cards, 13 each. Score as few points as possible. When someone reaches 500 the game ends — lowest total wins.",
    rules_spardame_points: [
        "Queen of spades = 100 points",
        "Ace of hearts = 20 points",
        "Every other heart = 10 points",
        "Jack of diamonds = −100 points",
    ],
    rules_spardame_extra:
        "The queen of spades can never be passed. Take every heart and the queen (a slam) and the other three each score 100. The jack of diamonds still counts for whoever took it.",
    rules_hearts_title: "Hearts (Windows)",
    rules_hearts_body:
        "Same flow as Hearts on Windows. Each heart is 1 point, the queen of spades is 13. Play to 100. The queen may be passed. Shoot the moon (all 13 hearts and the queen) and everyone else scores 26.",
    rules_play_title: "Play",
    rules_play: [
        "Pass 3 cards before each hand: left, right, across, then hold.",
        "Whoever holds the 2 of clubs leads it on the first trick.",
        "Follow suit if you can. Highest card of the suit led wins the trick.",
        "No penalty cards on the first trick (hearts or the queen of spades).",
        "Hearts cannot be led until they are broken — discarded because a player could not follow suit. The queen of spades may be led at any time.",
        "Play clockwise. No trump.",
    ],
    pass_left: "to the left",
    pass_right: "to the right",
    pass_across: "across",
    pass_hold: "no pass",
    cannot_pass_queen: "The queen of spades cannot be passed.",
    illegal: "You cannot play that card now.",
    pass_failed: "Could not pass those cards.",
    to: "to",
    games_won_pfx: "Games won",
    of: "of",
    send_pfx: "Pass",
    cards: "cards",
    pick_pfx: "Choose",
    got_pfx: "You received",
    you_take: "You take the trick.",
    takes_trick: "takes the trick",
    thinking: " is playing",
    follow_pfx: "Your turn — follow",
    void_pfx: "Your turn — you have no",
    won_sfx: "won",
    moon_sfx: "shot the moon — all hearts and the queen of spades.",
    score_hint_pfx: "First to",
    score_hint_sfx: "loses the game. Lowest total wins.",
    s_clubs: "clubs",
    s_diamonds: "diamonds",
    s_spades: "spades",
    s_hearts: "hearts",
    s_clubs_c: "Clubs",
    s_diamonds_c: "Diamonds",
    s_spades_c: "Spades",
    s_hearts_c: "Hearts",
    face_jack: "J",
    face_queen: "Q",
    face_king: "K",
    face_ace: "A",
    var_spardame_name: "Queen of Spades",
    var_spardame_short: "Standard",
    var_spardame_blurb:
        "Queen of spades 100, ace of hearts 20, other hearts 10, jack of diamonds −100. First to 500.",
    var_hjerter_name: "Hearts",
    var_hjerter_short: "Windows",
    var_hjerter_blurb:
        "Hearts 1, queen of spades 13. Shoot the moon gives 26 to the others. First to 100.",
};

const NB: Copy = Copy {
    lang_name: "Norsk",
    card_game: "Kortspill",
    app_name: "Spardame",
    tagline:
        "Unngå hjerter og spar dame. Ta ruter knekt. Fire spillere, tre motstandere, offisielle regler.",
    your_name: "Ditt navn",
    you: "Du",
    opponents: "Motstandere",
    easy: "Lett",
    normal: "Middels",
    hard: "Vanskelig",
    new_game: "Nytt parti",
    continue_: "Fortsett",
    rules: "Regler",
    sound_on: "Lyd på",
    sound_off: "Lyd av",
    card_back: "Kortbakside",
    card_back_tint_red: "Rød",
    card_back_tint_blue: "Blå",
    to_menu: "Til meny",
    close: "Lukk",
    language: "Språk",
    passing: "Bytte",
    lead: "Utspill",
    hearts_broken: "Hjerter brutt",
    queen_out: "Dame ude",
    jack_out: "Knekt ude",
    trick: "Stikk",
    hand_over: "Runden er ferdig.",
    game_over: "Partiet er over.",
    your_lead_two: "Din tur — spill ut kløver 2.",
    your_lead: "Din tur — spill ut.",
    playing: "Spill",
    queen_of_spades: "Spar dame",
    jack_of_diamonds: "Ruter knekt",
    score_hand: "Runden er ferdig",
    you_won: "Du vant",
    draw: "Uavgjort",
    scores: "Poeng",
    player: "Spiller",
    round: "Runde",
    total: "Sum",
    out: "ute",
    menu: "Meny",
    quit: "Avslutt",
    next_hand: "Neste runde",
    rules_title: "Regler",
    rules_spardame_title: "Spardame (norsk)",
    rules_spardame_body:
        "Fire spillere, 52 kort, 13 hver. Målet er å få færrest poeng. Første spiller til 500 avslutter partiet — lavest sum vinner.",
    rules_spardame_points: [
        "Spar dame = 100 poeng",
        "Hjerter ess = 20 poeng",
        "Øvrige hjerter = 10 poeng hver",
        "Ruter knekt = −100 poeng",
    ],
    rules_spardame_extra:
        "Spar dame kan aldri byttes bort. Tar du alle hjerter og spar dame (slem), får de tre andre 100 poeng hver. Ruter knekt telles likevel hos den som tok den.",
    rules_hearts_title: "Hjerter (Windows)",
    rules_hearts_body:
        "Samme spillgang som i Hearts på Windows. Hvert hjerterkort gir 1 poeng, spar dame gir 13. Partiet går til 100. Spar dame kan byttes bort. Skyter du månen (alle 13 hjerter og spar dame), får de andre 26 poeng hver.",
    rules_play_title: "Spillgang",
    rules_play: [
        "Bytt 3 kort før hver runde: venstre, høyre, over bordet, deretter hold.",
        "Den som har kløver 2 spiller den ut i første stikk.",
        "Følg farge hvis du kan. Høyeste kort i utspillfargen tar stikket.",
        "Ingen straffekort i første stikk (hjerter eller spar dame).",
        "Hjerter kan ikke spilles ut før de er brutt — kastet fordi noen ikke kunne følge farge. Spar dame kan spilles ut når som helst.",
        "Spill med klokken. Ingen trumf.",
    ],
    pass_left: "til venstre",
    pass_right: "til høyre",
    pass_across: "over bordet",
    pass_hold: "ingen bytte",
    cannot_pass_queen: "Spar dame kan ikke byttes bort.",
    illegal: "Det kortet kan du ikke spille nå.",
    pass_failed: "Kunne ikke bytte.",
    to: "til",
    games_won_pfx: "Partier vunnet",
    of: "av",
    send_pfx: "Send",
    cards: "kort",
    pick_pfx: "Velg",
    got_pfx: "Du fikk",
    you_take: "Du tar stikket.",
    takes_trick: "tar stikket",
    thinking: " spiller",
    follow_pfx: "Din tur — følg",
    void_pfx: "Din tur — du er blank i",
    won_sfx: "vant",
    moon_sfx: "tok slem — alle hjerter og spar dame.",
    score_hint_pfx: "Først til",
    score_hint_sfx: "poeng taper partiet. Lavest sum vinner.",
    s_clubs: "kløver",
    s_diamonds: "ruter",
    s_spades: "spar",
    s_hearts: "hjerter",
    s_clubs_c: "Kløver",
    s_diamonds_c: "Ruter",
    s_spades_c: "Spar",
    s_hearts_c: "Hjerter",
    face_jack: "Kn",
    face_queen: "D",
    face_king: "K",
    face_ace: "A",
    var_spardame_name: "Spardame",
    var_spardame_short: "Norsk",
    var_spardame_blurb:
        "Spar dame 100, hjerter ess 20, øvrige hjerter 10, ruter knekt −100. Til 500 poeng.",
    var_hjerter_name: "Hjerter",
    var_hjerter_short: "Windows",
    var_hjerter_blurb:
        "Hjerter 1, spar dame 13. Skyte månen gir 26 til de andre. Til 100 poeng.",
};
