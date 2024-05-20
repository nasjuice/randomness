use std::default;

use anchor_lang::prelude::*;

#[account]
pub struct House {
    pub bump: u8,
    pub max_guess: u8,
    pub authority: Pubkey,
    pub function: Pubkey,
    pub token_wallet: Pubkey
}

impl House {
    pub const SEEDS: &'static [u8] = b"house_";
    pub const SIZE: usize = 8 + 1 + 1 + 32 + 32 + 32;
}

#[derive(Default, Clone, AnchorDeserialize, AnchorSerialize)]
pub enum RoundStatus {
    #[default]
    None = 0,
    Pending,
    Settled,
}

impl From<RoundStatus> for u8 {
    fn from(value: RoundStatus) -> Self {
        match value {
            RoundStatus::Pending => 1,
            RoundStatus::Settled => 2,
            _ => 0,
        }
    }
}
impl From<u8> for RoundStatus {
    fn from(value: u8) -> Self {
        match value {
            1 => RoundStatus::Pending,
            2 => RoundStatus::Settled,
            _ => RoundStatus::default(),
        }
    }
}

#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub struct UserRound {
    pub request: Pubkey,
    pub guess: u8,
    pub status: RoundStatus,
    pub result: u8,
    pub wager: u64,
    pub slot: u64,
    pub timestamp: i64,
}

#[account]
pub struct UserState {
    pub bump: u8,
    pub authority: Pubkey,
    pub token_wallet: Pubkey,
    pub current_round: UserRound,
    pub last_round: UserRound,
}