use anchor_lang::prelude::*;

declare_id!("3CT4tE6xNnhpPgvXhXRnK2qZooXxvwRqrJMuVdFXVjeY");

#[program]
pub mod zkin_smartwallet {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
