use std::fmt;
use vec_map::VecMap;
use std::cmp::Ordering;
use ansi_term::Colour;

pub mod bits {
    const BIT_FLAG: u8 = 1 << 6;

    fn hide(card: u8) -> u8 { card | BIT_FLAG }

    fn show(card: u8) -> u8 { card & !BIT_FLAG }

    fn is_hidden(card: u8) -> bool { card & BIT_FLAG == BIT_FLAG }

    fn is_visible(card: u8) -> bool { !is_hidden(card) }
}

#[derive(Copy, Clone, PartialEq)]
pub enum Kind {
    Deuce,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Kind {
    pub const ALL: [Self; 13] = [Self::Deuce, Self::Three, Self::Four, Self::Five, Self::Six, Self::Seven, Self::Eight, Self::Nine, Self::Ten, Self::Jack, Self::Queen, Self::King, Self::Ace];

    pub fn short_name(&self) -> &str {
        match self{
            Self::Deuce => "2",
            Self::Three => "3",
            Self::Four => "4",
            Self::Five => "5",
            Self::Six => "6",
            Self::Seven => "7",
            Self::Eight => "8",
            Self::Nine => "9",
            Self::Ten => "T",
            Self::Jack => "J",
            Self::Queen => "Q",
            Self::King => "K",
            Self::Ace => "A",
        }
    }
    pub fn full_name(&self) -> &str {
        match self{
            Self::Deuce => "deuce",
            Self::Three => "three",
            Self::Four => "four",
            Self::Five => "five",
            Self::Six => "six",
            Self::Seven => "seven",
            Self::Eight => "eight",
            Self::Nine => "nine",
            Self::Ten => "ten",
            Self::Jack => "jack",
            Self::Queen => "queen",
            Self::King => "king",
            Self::Ace => "ace",
        }
    }
}

pub mod kind {
    use super::Kind;
    pub fn from_str(s: &str) -> Kind {
        match s {
            "2" => Kind::Deuce,
            "3" => Kind::Three,
            "4" => Kind::Four,
            "5" => Kind::Five,
            "6" => Kind::Six,
            "7" => Kind::Seven,
            "8" => Kind::Eight,
            "9" => Kind::Nine,
            "T" => Kind::Ten,
            "J" => Kind::Jack,
            "Q" => Kind::Queen,
            "K" => Kind::King,
            "A" => Kind::Ace,
            &_ => todo!(),
        }
    }
}

impl Default for Kind {
    fn default() -> Self { Kind::Ace }
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.short_name())
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum Suit {
    Spade,
    Heart,
    Diamond,
    Club,
}

impl Default for Suit {
    fn default() -> Self { Suit::Heart }
}

impl Suit {
    pub const ALL: [Self; 4] = [Self::Spade, Self::Heart, Self::Diamond, Self::Club];

    pub fn short_name(&self) -> &str {
        match self {
            Self::Spade => "s",
            Self::Heart => "h",
            Self::Diamond => "d",
            Self::Club => "c",
        }
    }

    pub fn full_name(&self) -> &str {
        match self {
            Self::Spade => "spade",
            Self::Heart => "heart",
            Self::Diamond => "diamond",
            Self::Club => "club",
        }
    }

    pub fn symbol(&self) -> &str {
        match self {
            Self::Spade => "♠",
            Self::Heart => "♥",
            Self::Diamond => "♦",
            Self::Club => "♣",
        }
    }

    pub fn color(&self) -> Colour {
        match self {
            Self::Spade => Colour::Black,
            Self::Heart => Colour::Red,
            Self::Diamond => Colour::Cyan,
            Self::Club => Colour::Green,
        }
    }
}
pub mod suit {
    use super::Suit;

    pub fn from_str(s: &str) -> Suit {
        match s {
            "s" => Suit::Spade,
            "h" => Suit::Heart,
            "d" => Suit::Diamond,
            "c" => Suit::Club,
            &_ => todo!(),
        }
    }
}

#[derive(Copy, Clone, Default)]
pub struct Card {
    kind: Kind,
    suit: Suit,
}

impl Card {
    pub fn to_u8(&self) -> u8 {
        return (self.kind as u8) << 2 + (self.suit as u8);
    }
}

impl fmt::Debug for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.kind.short_name(), self.suit.short_name())
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}",
            self.suit.color().on(Colour::White).paint(self.kind.short_name()),
            self.suit.color().on(Colour::White).paint(self.suit.symbol())
        )
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some((self.kind as u8).cmp(&(other.kind as u8)))
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind && self.suit == other.suit
    }
}

type Cards = Vec<Card>;

// impl fmt::Display for Cards {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{}", self.join(""))
//     }
// }

pub mod card {
    use regex::Regex;
    use super::{Card, Cards, kind, suit};

    pub fn new(kind: &str, suit: &str) -> Card {
        return Card { kind: kind::from_str(kind), suit: suit::from_str(suit) }
    }

    pub fn from_str(s: &str) -> Option<Card> {
        if s.len() != 2 {
            return None;
        }
        None
    }

    pub fn parse(s: &str) -> Cards {
        let regex = Regex::new(r"(?i)([akqjt2-9]{1})([shdc]{1})").unwrap();
        let mut cards: Cards = vec![];
        for cap in regex.captures_iter(s) {
            let card = new(&cap[1], &cap[2]);
            cards.push(card);
        }
        return cards
    }
}

pub mod rank {
    #[derive(Copy, Clone)]
    pub enum High {
        HighCard,
        OnePair,
        TwoPair,
        ThreeKind,
        Straight,
        Flush,
        FullHouse,
        FourKind,
        StraightFlush,
    }

    impl High {
        pub fn name(&self) -> &str {
            match self {
                Self::HighCard => "high-card",
                Self::OnePair => "one-pair",
                Self::TwoPair => "two-pair",
                Self::ThreeKind => "three-kind",
                Self::Straight => "straight",
                Self::Flush => "flush",
                Self::FullHouse => "full-house",
                Self::FourKind => "four-kind",
                Self::StraightFlush => "straight-flush",
            }
        }
    }

    #[derive(Copy, Clone)]
    pub enum Badugi {
        BadugiOne,
        BadugiTwo,
        BadugiThree,
        BadugiFour,
    }

    impl Badugi {
        pub fn name(&self) -> &str {
            match self {
                Self::BadugiOne => "badugi-one",
                Self::BadugiTwo => "badugi-two",
                Self::BadugiThree => "badugi-three",
                Self::BadugiFour => "badugi-four",
            }
        }
    }

    #[derive(Copy, Clone)]
    pub enum Low {
        CompleteLow,
        IncompleteLow,
    }

    impl Low {
        pub fn name(&self) -> &str {
            match self {
                Self::CompleteLow => "complete-low",
                Self::IncompleteLow => "incomplete-low",
            }
        }
    }

    use super::Rank;

    pub fn straight_flush() -> Rank {
        Rank{ high: High::StraightFlush }
    }

    pub fn four_kind() -> Rank {
        Rank{ high: High::FourKind }
    }
}

use std::cmp::Ordering::{Equal, Greater, Less};

use crate::game::AceRanking;

mod compare {
    use super::{Cards, Rank, rank};

    use std::cmp::Ordering;
    use std::cmp::Ordering::{Less, Equal, Greater};

    pub fn compare_ranks(a: &Rank, b: &Rank) -> Ordering {
        unsafe {
            match &a {
                Rank { high } => (*high as u8).cmp(&(b.high as u8)),
                Rank { low } => (*low as u8).cmp(&(b.low as u8)),
                Rank { badugi } => (*badugi as u8).cmp(&(b.badugi as u8)),
            }
        }
    }

    pub fn compare_cards(a: &Cards, b: &Cards) -> Ordering {
        if a.len() == b.len() {
            let mut result = Equal;
            //a.iter().for_each( |card| result = card.cmp(b[i]); result == Equal );
            return result;
        }
        Equal

        // let min_size = a.len().min(b.len());
        // compare_cards(a.iter().take(min_size).collect().to_vec(), b.iter().take(min_size).collect().to_vec())
    }
}

#[derive(Clone)]
pub struct Hand<'a> {
    cards: &'a Cards,
    value: &'a Cards,
    rank: Option<Rank>,
    high: Result<&'a Cards, bool>,
    kicker: Result<&'a Cards, bool>,
}

impl Hand<'_> {
    pub fn meaningful_high(&self) -> &Cards {
        match self.high {
            Ok(cards) => cards,
            Err(true) => &self.cards,
            Err(false) => &self.cards,
        }
    }

    pub fn meaningful_kicker(&self) -> &Cards {
        match self.kicker {
            Ok(cards) => cards,
            Err(true) => &self.cards,
            Err(false) => &self.cards,
        }
    }

    pub fn description(&self) -> String {
        unsafe {
            match self.rank {
                Some(rank) => match rank {
                    Rank{ high: rank::High::HighCard } => format!("high card {}", self.cards[0].kind),
                    Rank{ high: rank::High::OnePair } => format!("pair of {}s", self.cards[0].kind),
                    Rank{ high: rank::High::TwoPair } => format!("two pairs, {}s and {}s", self.cards[0].kind, self.cards[1].kind),
                    Rank{ high: rank::High::ThreeKind } => format!("three of a kind, {}s", self.cards[0].kind),
                    // Rank{ high: rank::High::Straight } => {
                    //     let (from, to) = if self.cards[0].kind == Kind::Five {
                    //         (self.value.min(AceRanking::Low).kind, self.value.max(AceRanking::Low).kind)
                    //     } else {
                    //         (self.value.min().kind, self.value.max().kind)
                    //     };
                    //     format!("straight, {} to {}", from, to)
                    // }
                    Rank{ high: rank::High::Flush } => format!("flush, {} high", self.cards[0].kind),
                    Rank{ high: rank::High::FullHouse } => format!("full house, {}s full of {}s", self.cards[0].kind, self.cards[1].kind),
                    Rank{ high: rank::High::FourKind } => format!("four of a kind, {}s", self.cards[0].kind),
                    //Rank{ high: rank::High::StraightFlush } => format!("straight flush, {} to {}", self.value.min().kind, self.value.max().kind),

                    Rank{ badugi: rank::Badugi::BadugiOne } => format!("1-card badugi: {}", self.value[0]),
                    Rank{ badugi: rank::Badugi::BadugiTwo } => format!("2-card badugi: {} + {}", self.value[0], self.value[1]),
                    Rank{ badugi: rank::Badugi::BadugiThree } => format!("3-card badugi: {} + {} + {}", self.value[0], self.value[1], self.value[2]),
                    Rank{ badugi: rank::Badugi::BadugiFour } => format!("4-card badugi: {} + {} + {} + {}", self.value[0], self.value[1], self.value[2], self.value[3]),

                    _ => todo!() //format!("(unknown: {})", self),
                },
                _ => "(none)".to_string(),
            }
        }
    }
}

impl PartialOrd for Hand<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let mut result: Ordering = Equal;

        match (&self.rank, &other.rank) {
            (Some(rank1), Some(rank2)) =>
                result = compare::compare_ranks(rank1, rank2),
            (Some(rank1), None) =>
                result = Greater,
            (None, Some(rank2)) =>
                result = Equal,
            (None, None) =>
                result = Less,
        }
        
        if result != Equal {
            return Some(result);
        }

        result = compare::compare_cards(self.meaningful_high(), other.meaningful_high());
        if result != Equal {
            return Some(result);
        }

        result = compare::compare_cards(&self.value, &other.value);
        if result != Equal {
            return Some(result);
        }

        result = compare::compare_cards(self.meaningful_kicker(), other.meaningful_kicker());
        if result != Equal {
            return Some(result);
        }

        Some(Equal)
    }
}

impl PartialEq for Hand<'_> {
    fn eq(&self, other: &Self) -> bool {
        true
    }
}

#[derive(Clone, Copy)]
pub union Rank {
    high: rank::High,
    badugi: rank::Badugi,
    low: rank::Low,
}

trait HighHand {
    fn new_hand(&self, hand: Hand) -> &Self;
    fn repetitions(&self) -> VecMap<Vec<&Cards>>;
    fn is_straight_flush(&self) -> Option<Hand> {
        match self.is_flush() {
            Some(hand) => {
                let mut hand2 = hand.clone();
                hand2.cards = hand.value;
                hand2.rank = Some(rank::straight_flush());
                return self.new_hand(hand2).is_straight().or_else( ||
                    self.is_full_house().or_else( ||
                        self.is_straight()
                    )
                );
            },
            None =>
                self.is_four_kind().or_else( ||
                    self.is_full_house().or_else( ||
                        self.is_straight()
                    )
                ),
        }
    }

    fn is_four_kind(&self) -> Option<Hand> {
        match self.repetitions().get(4) {
            Some(card_group) => {
                let cards = card_group.first().unwrap();
                return Some(
                    Hand { cards, value: cards, rank: Some(rank::four_kind()), high: Ok(cards), kicker: Err(true) }
                )
            },
            None => None,
        }
    }

    fn is_full_house(&self) -> Option<Hand> {
        // match self.repetitions().get(3) {

        // }
        // val (major, minor) = if (sets.size > 1) {
        //     val Seq(minor, major, _*) = sets.sorted(CardOrdering.ByHead).reverse.take(2)
        //     (major, minor)
        //   } else {
        //     paired.get(2) match {
        //       case None ⇒ return None
        //       case Some(pairs) ⇒
        //         val minor = pairs.sorted(CardOrdering.ByHead).reverse.head
        //         val major = sets.head
        //         (major, minor)
        //     }
        //   }
        //   hand(
        //       value   = major ++ minor,
        //       high    = Left(List(major head, minor head))
        //     ).map(_.ranked(FullHouse))
        None
    }

    fn is_flush(&self) -> Option<Hand> { return None; }

    fn is_straight(&self) -> Option<Hand> { return None; }

    fn is_three_kind(&self) -> Option<Hand> { return None; }

    fn is_two_pair(&self) -> Option<Hand> { return None; }

    fn is_one_pair(&self) -> Option<Hand> { return None; }

    fn is_high_card(&self) -> Option<Hand> { return None; }

    fn is_high(&self) -> Option<Hand> { return None; }
}

trait Badugi {
    fn is_badugi_four(&self) -> Option<Hand> {
        return None; 
    }

    fn is_badugi_three(&self) -> Option<Hand> {
        return None;
    }

    fn is_badugi_two(&self) -> Option<Hand> {
        return None;
    }

    fn is_badugi_one(&self) -> Option<Hand> {
        return None; 
    }
}

trait LowHand {
    fn is_low() -> Option<Hand<'static>> { return None; }
    fn is_gap_low() -> Option<Hand<'static>> { return None; }
}

mod deck {
    use rand::seq::SliceRandom;
    use rand::thread_rng;
    use super::{Cards, Card, Kind, Suit};

    const BOARD_SIZE: u8 = 5;

    pub fn new() -> Cards {
        let mut result: Cards = vec![];
        for suit in Suit::ALL {
            for kind in Kind::ALL {
                result.push(Card{ kind, suit });
            }
        }
        result
    }

    pub fn shuffle(mut cards: Cards) -> Cards {
        let mut rng = thread_rng();
        cards.shuffle(&mut rng);
        cards
    }

    pub fn default() -> Cards { shuffle(new()) }
}

#[derive(Debug)]
pub struct Deck {
    cards: Cards,
    dealt: Cards,
    discarded: Cards,
    burned: Cards,
}

impl Deck {
    pub fn new() -> Self {
        Deck {
            cards: deck::default(),
            dealt: vec![],
            discarded: vec![],
            burned: vec![],
        }
    }

    pub fn drop(&mut self, n: usize) -> Cards {
        self.cards.drain(0..n).collect()
    }

    pub fn reshuffle(&mut self) {
        self.cards.append(&mut self.burned);
        self.burned = vec![];
        self.cards = deck::shuffle(self.cards.clone());
    }

    pub fn deal(&mut self, n: usize) -> Cards {
        let cards = self.drop(n);
        self.dealt.append(&mut cards.clone());
        cards
    }

    pub fn burn(&mut self, n: usize) {
        let dropped = &mut self.drop(n);
        self.burned.append(dropped);
    }

    pub fn discard(&mut self, old_cards: &mut Cards) -> Cards {
        let n = old_cards.len();
        if n > self.cards.len() + self.burned.len() {
            //panic!("no cards left");
        }

        if n > self.cards.len() {
            self.reshuffle()
        }

        let cards = self.deal(n);
        self.burned.append(old_cards);

        let mut i = 0;
        while i < self.dealt.len() {
            let el = self.dealt[i];
            if old_cards.contains(&el) {
                self.dealt.remove(i);
            } else {
                i += 1;
            }
        }

        cards
    }
}

pub struct Dealer<'a> {
    deck: Deck,
    board_cards: &'a mut Cards,
    pocket_cards: &'a mut Vec<Cards>,
}

impl Dealer<'_> {
    pub fn deal_pocket_cards(&mut self, position: usize, cards_num: usize) -> Cards {
        let cards = self.deck.deal(cards_num);
        self.pocket_cards[position].append(&mut cards.clone());
        cards
    }

    pub fn deal_board_cards(&mut self, cards_num: usize) -> Cards {
        self.deck.burn(1);
        let cards = self.deck.deal(cards_num);
        self.board_cards.append(&mut cards.clone());
        cards
    }

    pub fn discard_cards(&self, cards: Cards, position: usize) -> Cards {
        self.pocket_cards[position].clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn parse_cards() {
        let cards = card::parse("AhTd3s7c");
        println!("{:?}", cards);
    }

    #[test]
    pub fn shuffle_cards() {
        println!("{:?}", deck::default());
    }
}
