import type { Flash, PassDir, Suit } from "./types.ts";

export type Locale = "en" | "nb";

export const LOCALES: { id: Locale; label: string }[] = [
  { id: "en", label: "EN" },
  { id: "nb", label: "Norsk" },
];

type Strings = {
  langName: string;
  cardGame: string;
  appName: string;
  tagline: string;
  yourName: string;
  you: string;
  opponents: string;
  easy: string;
  normal: string;
  hard: string;
  newGame: string;
  continue: string;
  rules: string;
  soundOn: string;
  soundOff: string;
  toMenu: string;
  close: string;
  language: string;
  gamesWon: (won: number, played: number) => string;
  toLimit: (n: number) => string;
  variant: Record<"spardame" | "hjerter", { name: string; short: string; blurb: string }>;
  pass: Record<PassDir, string>;
  sendCards: (n: number, of: number, dir: string) => string;
  pickCards: (n: number, dir: string) => string;
  pickCardsShort: (n: number) => string;
  gotCards: (n: number, from: string) => string;
  cannotPassQueen: string;
  illegal: string;
  passFailed: string;
  passing: string;
  lead: string;
  heartsBroken: string;
  queenOut: string;
  jackOut: string;
  trick: string;
  takesTrick: (name: string, isYou?: boolean) => string;
  handOver: string;
  gameOver: string;
  thinking: (name: string) => string;
  yourLeadTwo: string;
  yourLead: string;
  followSuit: (suit: string) => string;
  voidIn: (suit: string) => string;
  playing: string;
  suit: Record<Suit, string>;
  suitCap: Record<Suit, string>;
  cardBack: string;
  queenOfSpades: string;
  jackOfDiamonds: string;
  rank: { jack: string; queen: string; king: string; ace: string };
  face: { jack: string; queen: string; king: string; ace: string };
  scoreHand: string;
  youWon: string;
  won: (name: string) => string;
  draw: string;
  scores: string;
  player: string;
  round: string;
  total: string;
  out: string;
  menu: string;
  quit: string;
  nextHand: string;
  moon: (name: string) => string;
  scoreHint: (limit: number) => string;
  rulesTitle: string;
  rulesSpardameTitle: string;
  rulesSpardameBody: string;
  rulesSpardamePoints: string[];
  rulesSpardameExtra: string;
  rulesHeartsTitle: string;
  rulesHeartsBody: string;
  rulesPlayTitle: string;
  rulesPlay: string[];
};

export const STRINGS: Record<Locale, Strings> = {
  en: {
    langName: "English",
    cardGame: "Card game",
    appName: "Hearts",
    tagline:
      "Avoid hearts and the queen of spades. Take the jack of diamonds. Four players, three opponents, official rules.",
    yourName: "Your name",
    you: "You",
    opponents: "Opponents",
    easy: "Easy",
    normal: "Medium",
    hard: "Hard",
    newGame: "New game",
    continue: "Continue",
    rules: "Rules",
    soundOn: "Sound on",
    soundOff: "Sound off",
    toMenu: "Back to menu",
    close: "Close",
    language: "Language",
    gamesWon: (won, played) => `Games won: ${won} of ${played}`,
    toLimit: (n) => `to ${n}`,
    variant: {
      spardame: {
        name: "Hearts",
        short: "Standard",
        blurb:
          "Queen of spades 100, ace of hearts 20, other hearts 10, jack of diamonds −100. First to 500.",
      },
      hjerter: {
        name: "Hearts",
        short: "Windows",
        blurb: "Hearts 1, queen of spades 13. Shoot the moon gives 26 to the others. First to 100.",
      },
    },
    pass: {
      left: "to the left",
      right: "to the right",
      across: "across",
      hold: "no pass",
    },
    sendCards: (n, of, dir) => `Pass ${n}/${of} cards ${dir}`,
    pickCards: (n, dir) => `Choose ${n} cards to pass ${dir}.`,
    pickCardsShort: (n) => `Choose ${n} cards to pass.`,
    gotCards: (n, from) => `You received ${n} cards from ${from}.`,
    cannotPassQueen: "The queen of spades cannot be passed.",
    illegal: "You cannot play that card now.",
    passFailed: "Could not pass those cards.",
    passing: "Passing",
    lead: "Lead",
    heartsBroken: "Hearts broken",
    queenOut: "Queen out",
    jackOut: "Jack out",
    trick: "Trick",
    takesTrick: (name, isYou) => (isYou ? "You take the trick." : `${name} takes the trick.`),
    handOver: "Hand complete.",
    gameOver: "Game over.",
    thinking: (name) => `${name} is playing…`,
    yourLeadTwo: "Your lead — play the 2 of clubs.",
    yourLead: "Your lead — play a card.",
    followSuit: (suit) => `Your turn — follow ${suit}.`,
    voidIn: (suit) => `Your turn — you have no ${suit}.`,
    playing: "Playing",
    suit: { clubs: "clubs", diamonds: "diamonds", spades: "spades", hearts: "hearts" },
    suitCap: { clubs: "Clubs", diamonds: "Diamonds", spades: "Spades", hearts: "Hearts" },
    cardBack: "Card back",
    queenOfSpades: "Queen of spades",
    jackOfDiamonds: "Jack of diamonds",
    rank: { jack: "jack", queen: "queen", king: "king", ace: "ace" },
    face: { jack: "J", queen: "Q", king: "K", ace: "A" },
    scoreHand: "Hand complete",
    youWon: "You won",
    won: (name) => `${name} won`,
    draw: "Draw",
    scores: "Scores",
    player: "Player",
    round: "Hand",
    total: "Total",
    out: "out",
    menu: "Menu",
    quit: "Quit",
    nextHand: "Next hand",
    moon: (name) => `${name} shot the moon — all hearts and the queen of spades.`,
    scoreHint: (limit) => `First to ${limit} loses the game. Lowest total wins.`,
    rulesTitle: "Rules",
    rulesSpardameTitle: "Hearts",
    rulesSpardameBody:
      "Four players, 52 cards, 13 each. Score as few points as possible. When someone reaches 500 the game ends — lowest total wins.",
    rulesSpardamePoints: [
      "Queen of spades = 100 points",
      "Ace of hearts = 20 points",
      "Every other heart = 10 points",
      "Jack of diamonds = −100 points",
    ],
    rulesSpardameExtra:
      "The queen of spades can never be passed. Take every heart and the queen (a slam) and the other three each score 100. The jack of diamonds still counts for whoever took it.",
    rulesHeartsTitle: "Hearts (Windows)",
    rulesHeartsBody:
      "Same flow as Hearts on Windows. Each heart is 1 point, the queen of spades is 13. Play to 100. The queen may be passed. Shoot the moon (all 13 hearts and the queen) and everyone else scores 26.",
    rulesPlayTitle: "Play",
    rulesPlay: [
      "Pass 3 cards before each hand: left, right, across, then hold.",
      "Whoever holds the 2 of clubs leads it on the first trick.",
      "Follow suit if you can. Highest card of the suit led wins the trick.",
      "No penalty cards on the first trick (hearts or the queen of spades).",
      "Hearts cannot be led until they are broken — discarded because a player could not follow suit. The queen of spades may be led at any time.",
      "Play clockwise. No trump.",
    ],
  },
  nb: {
    langName: "Norsk",
    cardGame: "Kortspill",
    appName: "Spardame",
    tagline:
      "Unngå hjerter og spar dame. Ta ruter knekt. Fire spillere, tre motstandere, offisielle regler.",
    yourName: "Ditt navn",
    you: "Du",
    opponents: "Motstandere",
    easy: "Lett",
    normal: "Middels",
    hard: "Vanskelig",
    newGame: "Nytt parti",
    continue: "Fortsett",
    rules: "Regler",
    soundOn: "Lyd på",
    soundOff: "Lyd av",
    toMenu: "Til meny",
    close: "Lukk",
    language: "Språk",
    gamesWon: (won, played) => `Partier vunnet: ${won} av ${played}`,
    toLimit: (n) => `til ${n}`,
    variant: {
      spardame: {
        name: "Spardame",
        short: "Norsk",
        blurb: "Spar dame 100, hjerter ess 20, øvrige hjerter 10, ruter knekt −100. Til 500 poeng.",
      },
      hjerter: {
        name: "Hjerter",
        short: "Windows",
        blurb: "Hjerter 1, spar dame 13. Skyte månen gir 26 til de andre. Til 100 poeng.",
      },
    },
    pass: {
      left: "til venstre",
      right: "til høyre",
      across: "over bordet",
      hold: "ingen bytte",
    },
    sendCards: (n, of, dir) => `Send ${n}/${of} kort ${dir}`,
    pickCards: (n, dir) => `Velg ${n} kort å sende ${dir}.`,
    pickCardsShort: (n) => `Velg ${n} kort å sende.`,
    gotCards: (n, from) => `Du fikk ${n} kort fra ${from}.`,
    cannotPassQueen: "Spar dame kan ikke byttes bort.",
    illegal: "Det kortet kan du ikke spille nå.",
    passFailed: "Kunne ikke bytte.",
    passing: "Bytte",
    lead: "Utspill",
    heartsBroken: "Hjerter brutt",
    queenOut: "Dame ude",
    jackOut: "Knekt ude",
    trick: "Stikk",
    takesTrick: (name) => `${name} tar stikket.`,
    handOver: "Runden er ferdig.",
    gameOver: "Partiet er over.",
    thinking: (name) => `${name} spiller…`,
    yourLeadTwo: "Din tur — spill ut kløver 2.",
    yourLead: "Din tur — spill ut.",
    followSuit: (suit) => `Din tur — følg ${suit}.`,
    voidIn: (suit) => `Din tur — du er blank i ${suit}.`,
    playing: "Spill",
    suit: { clubs: "kløver", diamonds: "ruter", spades: "spar", hearts: "hjerter" },
    suitCap: { clubs: "Kløver", diamonds: "Ruter", spades: "Spar", hearts: "Hjerter" },
    cardBack: "Kort bakside",
    queenOfSpades: "Spar dame",
    jackOfDiamonds: "Ruter knekt",
    rank: { jack: "knekt", queen: "dame", king: "konge", ace: "ess" },
    face: { jack: "Kn", queen: "D", king: "K", ace: "A" },
    scoreHand: "Runden er ferdig",
    youWon: "Du vant",
    won: (name) => `${name} vant`,
    draw: "Uavgjort",
    scores: "Poeng",
    player: "Spiller",
    round: "Runde",
    total: "Sum",
    out: "ute",
    menu: "Meny",
    quit: "Avslutt",
    nextHand: "Neste runde",
    moon: (name) => `${name} tok slem — alle hjerter og spar dame.`,
    scoreHint: (limit) => `Først til ${limit} poeng taper partiet. Lavest sum vinner.`,
    rulesTitle: "Regler",
    rulesSpardameTitle: "Spardame (norsk)",
    rulesSpardameBody:
      "Fire spillere, 52 kort, 13 hver. Målet er å få færrest poeng. Første spiller til 500 avslutter partiet — lavest sum vinner.",
    rulesSpardamePoints: [
      "Spar dame = 100 poeng",
      "Hjerter ess = 20 poeng",
      "Øvrige hjerter = 10 poeng hver",
      "Ruter knekt = −100 poeng",
    ],
    rulesSpardameExtra:
      "Spar dame kan aldri byttes bort. Tar du alle hjerter og spar dame (slem), får de tre andre 100 poeng hver. Ruter knekt telles likevel hos den som tok den.",
    rulesHeartsTitle: "Hjerter (Windows)",
    rulesHeartsBody:
      "Samme spillgang som i Hearts på Windows. Hvert hjerterkort gir 1 poeng, spar dame gir 13. Partiet går til 100. Spar dame kan byttes bort. Skyter du månen (alle 13 hjerter og spar dame), får de andre 26 poeng hver.",
    rulesPlayTitle: "Spillgang",
    rulesPlay: [
      "Bytt 3 kort før hver runde: venstre, høyre, over bordet, deretter hold.",
      "Den som har kløver 2 spiller den ut i første stikk.",
      "Følg farge hvis du kan. Høyeste kort i utspillfargen tar stikket.",
      "Ingen straffekort i første stikk (hjerter eller spar dame).",
      "Hjerter kan ikke spilles ut før de er brutt — kastet fordi noen ikke kunne følge farge. Spar dame kan spilles ut når som helst.",
      "Spill med klokken. Ingen trumf.",
    ],
  },
};

export function t(locale: Locale): Strings {
  return STRINGS[locale] ?? STRINGS.en;
}

export function flashCopy(locale: Locale, flash: Flash): string {
  const copy = t(locale);
  switch (flash.kind) {
    case "cannotPassQueen":
      return copy.cannotPassQueen;
    case "pickCards":
      return copy.pickCardsShort(flash.n);
    case "gotCards":
      return copy.gotCards(flash.n, flash.from);
    case "illegal":
      return copy.illegal;
    case "passFailed":
      return copy.passFailed;
  }
}

export function htmlLang(locale: Locale): "en" | "nb" {
  return locale === "nb" ? "nb" : "en";
}
