use anchor_lang::prelude::*;

declare_id!("7rppEp63Wc6gyoPEwtaAaqAu3MYwcDNoNor2Ha3izQmQ");

#[program]
pub mod contract {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}