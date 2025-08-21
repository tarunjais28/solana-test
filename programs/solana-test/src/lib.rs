use anchor_lang::prelude::*;

declare_id!("Bzibkqbrfq2tT8FmT2DfRuBP8MBw3ViH9MtQurm3abrN");

#[program]
pub mod solana_test {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
