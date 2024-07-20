use anchor_lang::prelude::*;

declare_id!("Fk8Ht5g2mWbSBbbxU7kn6Ss7LisBnQPDZK6k5Mzn9Cre");

#[program]
pub mod solana_lottery {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
