use anchor_lang::prelude::*;

declare_id!("98D2zgVthDCsZcF6gRMLrwfoptTaYiVCGC92pV5fXwZ8");

#[program]
pub mod prop_pool {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
