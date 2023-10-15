use crate::player::Player;
use crate::seat::Seat;
use crate::pot::Pot;
use std::fmt;

#[derive(Debug)]
pub struct Table<'a> {
    size: u8,
    button: u8,
    seats: Vec<Seat<'a>>,
    pot: Pot,
}

impl Table<'_> {
    pub fn new(size: u8) -> Self {
        Table {
            size,
            button: 0,
            seats: vec![],
            pot: Pot{},
        }
    }

    pub fn join(&self, player: Player, position: u8, amount: u16) {

    }
}

impl fmt::Display for Table<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub type HeadsUp<'a> = [Seat<'a>; 2];
pub type FourMax<'a> = [Seat<'a>; 4];
pub type SixMax<'a> = [Seat<'a>; 6];
pub type EightMax<'a> = [Seat<'a>; 8];
pub type NineMax<'a> = [Seat<'a>; 9];
pub type TenMax<'a> = [Seat<'a>; 10];
