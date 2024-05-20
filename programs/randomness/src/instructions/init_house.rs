use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Mint, Token, TokenAccount};
use switchboard_solana::prelude::{FunctionAccountData, SwitchboardWallet};

use crate::state::House;

#[derive(Accounts)]
pub struct InitHouse<'info> {

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,

    #[account(init, payer = authority, 
        space = House::SIZE,
        seeds = [House::SEEDS],
        bump
    )]
    pub house: Account<'info, House>,

    // SWITCHBOARD ACCOUNTS
    #[account(
        has_one = escrow_wallet,
    )]
    pub function: AccountLoader<'info, FunctionAccountData>,

    #[account(has_one = token_wallet, has_one = mint)]
    pub escrow_wallet: Box<Account<'info, SwitchboardWallet>>,

    #[account(constraint = token_wallet.mint == mint.key())]
    pub token_wallet: Box<Account<'info, TokenAccount>>,
      
    // TOKEN ACCOUNTS
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,

    #[account(address = anchor_spl::token::spl_token::native_mint::ID)]
    pub mint: Account<'info, Mint>,

    #[account(
        init,
        payer = authority,
        associated_token::mint = mint,
        associated_token::authority = house,
    )]
    pub house_token_wallet: Box<Account<'info, TokenAccount>>,

}

pub fn handler(ctx: Context<InitHouse>, max_guess: u8) -> Result<()> {

    let house = &mut ctx.accounts.house;

    house.bump = *ctx.bumps.get("house").unwrap();
    house.max_guess = max_guess;
    house.authority = ctx.accounts.authority.key();
    house.function = ctx.accounts.function.key();
    house.token_wallet = ctx.accounts.house_token_wallet.key();

    Ok(())

}