use std::time;
use crate::player::Player;
use crate::bet::Bet;
use crate::bet::AutoPlay;

#[derive(Debug)]
enum State<'a> {
    Empty,
    Reserved(Player<'a>),
    Ready(Slot<'a>),
    WaitBigBlind(Slot<'a>),
    PostBigBlind(Slot<'a>, u16),
    Playing(Slot<'a>, Bet),
    AutoPlaying(Slot<'a>, AutoPlay),
    AllIn(Slot<'a>, Bet),
    Betting(Slot<'a>, Bet),
    Folding(Slot<'a>),
    SittingOut(Slot<'a>),
    Idle(Slot<'a>),
    Away(Slot<'a>),
    Gone(Slot<'a>),
}

impl State<'_> {
    pub fn check(self) -> Self {
        match self {
            Self::Playing(slot, _bet) => Self::Betting(slot,  Bet::Check),
            _ => self,
        }
    }

    pub fn fold(self) -> Self {
        match self {
            Self::Playing(slot, _bet) => Self::Folding(slot),
            _ => self,
        }
    }

    pub fn raise(self, amount: u32) -> Self {
        match self {
            Self::Playing(slot, bet) => Self::Betting(slot,  Bet::Raise(amount)),
            _ => self,
        }
    }

    pub fn call(self, amount: u32) -> Self {
        match self {
            Self::Playing(slot, bet) => Self::Betting(slot,  Bet::Call(amount)),
            _ => self,
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum NetworkStatus {
    Offline,
    Online,
}

#[derive(Debug)]
struct Slot<'a> {
    player: Player<'a>,
    network_status: NetworkStatus,
    current_stack_amount: u16,
    amount_put: u16,
    rebuy_amount: u16,
    clock: time::Duration,
    auto_play: AutoPlay,
}

impl Slot<'_> {
    pub fn offline(&mut self) {
        self.network_status = NetworkStatus::Offline;
    }

    pub fn online(&mut self) {
        self.network_status = NetworkStatus::Online;
    }

    pub fn toggle_network_status(&mut self) {
        self.network_status =
            if self.network_status == NetworkStatus::Offline {
                NetworkStatus::Online
            } else {
                NetworkStatus::Offline
            };
    }
}

#[derive(Debug)]
pub struct Seat<'a> {
    state: State<'a>,
}
