use anchor_lang::prelude::*;

declare_id!("5cJ9ib6ja2nsmuwGLNn9Kv3wk2R42S24gbnDsRrN3Abr");

pub use instructions::*;

pub mod instructions;
pub mod state;


#[program]
pub mod randomness {
    use super::*;

    pub fn init_house(ctx: Context<InitHouse>, max_guess: u8) -> Result<()> {
        init_house::handler(ctx, max_guess)
    }
}