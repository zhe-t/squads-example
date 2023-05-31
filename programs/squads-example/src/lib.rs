use anchor_lang::prelude::*;

declare_id!("9eEoyFmUVNwLiBrUKqxWdZ9jhrmVNtYnJprXC21nnn43");

#[program]
pub mod squads_example {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
