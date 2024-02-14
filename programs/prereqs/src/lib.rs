use anchor_lang::prelude::*;

declare_id!("8WgvuFLgrEbat7G3sMH2FhgtEUARm7cW8xmAt2fRYRRJ");

#[program]
pub mod prereqs {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
