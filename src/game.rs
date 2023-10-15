use crate::{table::Table, poker::Deck, bet::Action, locale::Currency};
use std::fmt;
use std::fmt::{Display};

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum Limit {
    #[default]
    FixedLimit,
    PotLimit,
    NoLimit,
}

impl Limit {
    pub fn full_name(&self) -> &str {
        match self {
            Self::FixedLimit => "Fixed Limit",
            Self::PotLimit => "Pot Limit",
            Self::NoLimit => "No Limit",
        }
    }

    pub fn short_name(&self) -> &str {
        match self {
            Self::FixedLimit => "FL",
            Self::PotLimit => "PL",
            Self::NoLimit => "NL",
        }
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum Name {
    #[default]
    Texas,
    Omaha,
    Omaha8,
    Stud,
    Stud8,
    Razz,
    London,
    FiveCard,
    Single27,
    Triple27,
    Badugi,
}

impl Name {
    pub fn human_readable_name(&self) -> &str {
        match self {
            Self::Texas => "Texas Hold'em",
            Self::Omaha => "Omaha",
            Self::Omaha8 => "Omaha Hi/Lo",
            Self::Stud => "Stud",
            Self::Stud8 => "Stud Hi/Lo",
            Self::Razz => "Razz",
            Self::London => "London",
            Self::FiveCard => "5-card Draw",
            Self::Single27 => "Single 2-7 Draw",
            Self::Triple27 => "Triple 2-7 Draw",
            Self::Badugi => "Badugi",
        }
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub enum Mix {
    #[default]
    EightGame,
    Horse,
}

type Type = (Name, Limit);

pub const EIGHT_GAME: [Type; 8] = [
    (Name::Triple27, Limit::FixedLimit),
    (Name::Texas, Limit::FixedLimit),
    (Name::Omaha8, Limit::FixedLimit),
    (Name::Razz, Limit::FixedLimit),
    (Name::Stud, Limit::FixedLimit),
    (Name::Stud8, Limit::FixedLimit),
    (Name::Texas, Limit::NoLimit),
    (Name::Omaha, Limit::PotLimit),
];

pub const HORSE: [Type; 5] = [
    (Name::Texas, Limit::FixedLimit),
    (Name::Omaha8, Limit::FixedLimit),
    (Name::Razz, Limit::FixedLimit),
    (Name::Stud, Limit::FixedLimit),
    (Name::Stud8, Limit::FixedLimit),
];

impl Mix {
    pub fn games(&self) -> Vec<Type> {
        match self {
            Self::EightGame => EIGHT_GAME.to_vec(),
            Self::Horse => HORSE.to_vec(),
        }
    }

    pub fn first(&self) -> Type {
        self.games()[0]
    }

    pub fn next(&self, name: Name, limit: Limit) -> Option<Type> {
        let games = self.games();
        let first = games.first();

        let mut iter = games.iter();
        while let Some(&game) = iter.next() {
            if name == game.0 && limit == game.1 {
                if let Some(next_game) = iter.next() {
                    return Some(next_game.clone());
                } else {
                    return first.copied();
                }
            }
        }
    
        return None;
    }
}

#[derive(Default)]
pub enum Variant {
    #[default]
    Holdem,
    SevenCard,
    SingleDraw,
    TripleDraw,
}

#[derive(Default)]
pub struct Options {
    variant: Variant,
    blinds: bool,
    ante: bool,
    bring_in: bool,
    board: bool,
    vela: bool,
    discards: bool,
    reshuffle: bool,
    max_table_size: usize,
    pocket_cards_size: usize,
    streets_num: usize,
    hi_ranking: Option<AceRanking>,
    lo_ranking: Option<AceRanking>,
    default_limit: Limit,
}

impl Name {
    pub fn options(&self) -> Options {
        match self {
            Name::Texas => Options {
                variant: Variant::Holdem,
                board: true,
                blinds: true,
                max_table_size: 10,
                hi_ranking: Some(AceRanking::High),
                pocket_cards_size: 2,
                default_limit: Limit::NoLimit,

                ante: false, bring_in: false, vela: false, discards: false, reshuffle: false, streets_num: 4, lo_ranking: None,
            },
          
            Name::Omaha => Options {
                variant: Variant::Holdem,
                board: true,
                blinds: true,
                max_table_size: 10,
                pocket_cards_size: 4,
                hi_ranking: Some(AceRanking::High),
                default_limit: Limit::PotLimit,

                ante: false, bring_in: false, vela: false, discards: false, reshuffle: false, streets_num: 4, lo_ranking: None,
            },
        
            Name::Omaha8 => Options {
                variant: Variant::Holdem,
                board: true,
                blinds: true,
                max_table_size: 10,
                pocket_cards_size: 4,
                hi_ranking: Some(AceRanking::High),
                lo_ranking: Some(AceRanking::AceToFive8),
                default_limit: Limit::PotLimit,

                ante: false, bring_in: false, vela: false, discards: false, reshuffle: false, streets_num: 4,
            },
        
            Name::Stud => Options {
                variant: Variant::SevenCard,
                ante: true,
                bring_in: true,
                vela: true,
                max_table_size: 8,
                pocket_cards_size: 7,
                hi_ranking: Some(AceRanking::High),
                default_limit: Limit::FixedLimit,

                blinds: false, board: false, discards: false, reshuffle: false, streets_num: 6, lo_ranking: None,
            },
          
            Name::Stud8 => Options {
                variant: Variant::SevenCard,
                ante: true,
                bring_in: true,
                vela: true,
                max_table_size: 8,
                pocket_cards_size: 7,
                hi_ranking: Some(AceRanking::High),
                lo_ranking: Some(AceRanking::AceToFive8),
                default_limit: Limit::FixedLimit,

                blinds: false, board: false, discards: false, reshuffle: false, streets_num: 6,
            },
          
            Name::Razz => Options {
                variant: Variant::SevenCard,
                ante: true,
                bring_in: true,
                vela: true,
                max_table_size: 8,
                pocket_cards_size: 7,
                hi_ranking: Some(AceRanking::AceToFive),
                default_limit: Limit::FixedLimit,

                blinds: false, board: false, discards: false, reshuffle: false, streets_num: 6, lo_ranking: None,
            },
        
            Name::London => Options {
                variant: Variant::SevenCard,
                ante: true,
                bring_in: true,
                vela: true,
                max_table_size: 8,
                pocket_cards_size: 7,
                hi_ranking: Some(AceRanking::AceToSix),
                default_limit: Limit::FixedLimit,

                blinds: false, board: false, discards: false, reshuffle: false, streets_num: 6, lo_ranking: None,
            },
        
            Name::FiveCard => Options {
                variant: Variant::SingleDraw,
                blinds: true,
                discards: true,
                reshuffle: true,
                max_table_size: 6,
                pocket_cards_size: 5,
                streets_num: 1,
                hi_ranking: Some(AceRanking::High),
                default_limit: Limit::FixedLimit,

                ante: false, bring_in: false, board: false, vela: false, lo_ranking: None,
            },
        
            Name::Single27 => Options{
                variant: Variant::SingleDraw,
                blinds: true,
                discards: true,
                reshuffle: true,
                max_table_size: 6,
                pocket_cards_size: 5,
                streets_num: 1,
                hi_ranking: Some(AceRanking::DeuceToSeven),
                default_limit: Limit::FixedLimit,

                ante: false, bring_in: false, board: false, vela: false, lo_ranking: None,
            },

            Name::Triple27 => Options {
                variant: Variant::TripleDraw,
                blinds: true,
                discards: true,
                reshuffle: true,
                max_table_size: 6,
                pocket_cards_size: 5,
                streets_num: 3,
                hi_ranking: Some(AceRanking::DeuceToSeven),
                default_limit: Limit::FixedLimit,

                ante: false, bring_in: false, board: false, vela: false, lo_ranking: None,
            },
          
            Name::Badugi => Options {
                variant: Variant::TripleDraw,
                blinds: true,
                discards: true,
                reshuffle: true,
                max_table_size: 6,
                pocket_cards_size: 4,
                streets_num: 3,
                hi_ranking: Some(AceRanking::Badugi),
                default_limit: Limit::FixedLimit,

                ante: false, bring_in: false, board: false, vela: false, lo_ranking: None,
            },
        }
    }
}

pub enum AceRanking {
    High,
    Low,
    AceToFive,
    AceToFive8,
    AceToSix,
    DeuceToSeven,
    Badugi,
}

#[derive(Default, Debug)]
pub struct Stake {
    small_blind: u16,
    big_blind: u16,
    buy_in: (u16, u16),
    ante: Option<u16>,
    bring_in: Option<u16>,
    currency: Option<Currency>,
}

pub mod forced_bets {
    pub const ANTE: f32 = 0.1;
    pub const BRING_IN: f32 = 0.25;
    pub const SMALL_BLIND: f32 = 0.5;
    pub const BIG_BLIND: f32 = 1.0;
}

pub mod stacks {
    pub const DEFAULT: (u16, u16) = (20, 100);
    pub const SHORT_STACK: (u16, u16) = (20, 40);
    pub const DEEP_STACK: (u16, u16) = (100, 250);
}

impl Stake {
    pub fn new(big_blind: u16) -> Self {
        let mut new_stake = Self::default();
        new_stake.big_blind = big_blind;
        new_stake.small_blind = new_stake.big_blind / 2;
        new_stake.default_stack()
    }

    pub fn is_real_currency(self) -> bool {
        self.currency.is_some()
    }

    pub fn with_ante(mut self) -> Self {
        self.ante = Some(self.big_blind / 10);
        self
    }

    pub fn with_bring_in(mut self) -> Self {
        self.bring_in = Some(self.big_blind / 4);
        self
    }

    pub fn default_stack(mut self) -> Self {
        self.buy_in = (stacks::DEFAULT.0 * self.big_blind, stacks::DEFAULT.1 * self.big_blind);
        self
    }

    pub fn short_stack(mut self) -> Self {
        self.buy_in = (stacks::SHORT_STACK.0 * self.big_blind, stacks::SHORT_STACK.1 * self.big_blind);
        self
    }

    pub fn deep_stack(mut self) -> Self {
        self.buy_in = (stacks::DEEP_STACK.0 * self.big_blind, stacks::DEEP_STACK.1 * self.big_blind);
        self
    }
}

impl Display for Stake {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "${}/${}", self.small_blind, self.big_blind)
    }
}

pub mod street {
    pub enum Holdem {
        Preflop,
        Flop,
        Turn,
        River,
    }

    pub enum SevenCard {
        Second,
        Third,
        Fourth,
        Fifth,
        Sixth,
        Seventh,
    }

    pub enum Draw {
        Predraw,
        Draw,
        FirstDraw,
        SecondDraw,
        ThirdDraw,
    }

    use Holdem::*;
    use SevenCard::*;
    use Draw::*;

    const HOLDEM: [Holdem; 4] = [Preflop, Flop, Turn, River];
    const SEVEN_CARD: [SevenCard; 6] = [Second, Third, Fourth, Fifth, Sixth, Seventh];
    const DRAW: [Draw; 5] = [Predraw, Draw, FirstDraw, SecondDraw, ThirdDraw];
}

type Log<T> = Vec<T>;

#[derive(Debug)]
pub struct Game<'a> {
    game: Name,
    limit: Limit,
    mix: Option<Mix>,
    stake: Stake,
    table: Table<'a>,
    deck: Deck,
    actions: Log<Action<'a>>,
}

impl Game<'_> {
    pub fn new(name: Name, limit: Limit, stake: Stake, table_size: u8) -> Self {
        return Game {
            game: name,
            limit,
            mix: None,
            stake,
            table: Table::new(table_size),
            deck: Deck::new(),
            actions: vec![],
        };
    }

    pub fn mix(mix: Mix, stake: Stake, table_size: u8) -> Self {
        let first: (Name, Limit) = mix.first();
        return Game {
            game: first.0,
            limit: first.1,
            mix: Some(mix),
            stake,
            table: Table::new(table_size),
            deck: Deck::new(),
            actions: vec![],
        };
    }

    pub fn next_game(&mut self) -> Option<Type> {
        match self.mix {
            Some(mix) => {
                let next = mix.next(self.game, self.limit).unwrap();
                self.game = next.0;
                self.limit = next.1;
                Some(next)
            },
            None => None,
        }
    }

    pub fn start(&self) {

    }
}

impl Display for Game<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,
            "Game: {} {}\nStake: {}\n{:?}",
            self.game.human_readable_name(),
            self.limit.full_name(),
            self.stake,
            self.table,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::player::Player;

    #[test]
    pub fn next_game() {
        let stake = Stake::new(10);
        let mix = Mix::EightGame;
        let mut game = Game::mix(mix, stake, 2);
        for i in 1..100 {
            println!("{:?}", game.next_game());
        }
    }

    #[test]
    pub fn start_game() {
        let stake = Stake::new(10);
        let game = Game::new(Name::Texas, Limit::NoLimit, stake, 2);
        let player_1 = Player::blank_player("player-1");
        let player_2 = Player::blank_player("player-2");
        game.table.join(player_1, 0, 1000);
        game.table.join(player_2, 1, 1000);
        game.start();

        println!("{}", game);
    }
}
