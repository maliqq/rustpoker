use std::time;
use crate::player::Player;

#[derive(Debug)]
pub enum Bet {
    Check,
    Fold,
    Raise(u32),
    Call(u32),
    SmallBlind(u32),
    BigBlind(u32),
    Straddle(u32),
    Ante(u32),
    BringIn(u32),
    GuestBlind(u32),
}

#[derive(Debug)]
pub enum AutoPlay {
    Fold,
    Check,
    CallAny,
    CheckFold,
    FoldAnyBet,
}

#[derive(Debug)]
pub struct Action<'a> {
    player: Player<'a>,
    bet: Bet,
    time: time::SystemTime,
    all_in: bool,
    forced_bet: bool,
    timed_out: bool,
}
