#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;

declare_id!("98D2zgVthDCsZcF6gRMLrwfoptTaYiVCGC92pV5fXwZ8");

mod constants;
mod errors;
mod ix;
mod state;

pub use ix::*;
pub use state::*;

#[program]
pub mod prop_pool {
    use super::*;

    // 1. Create Staging Proposal
    pub fn create_staged_proposal(
        ctx: Context<CreateStagedProp>,
        args: CreateStagedPropArgs,
    ) -> Result<()> {
        ix::ix_create_staged_prop(ctx, args)
    }

    // 2. Fund Staging Proposals
    pub fn fund_staged_proposal(
        ctx: Context<FundStagedProp>,
        args: FundStagedPropArgs,
    ) -> Result<()> {
        ix::ix_fund_staged_prop(ctx, args)
    }

    // 3. Withdraw from Staging Proposals
    pub fn withdraw_from_staged_proposal(
        ctx: Context<WithdrawFromStagedProp>,
        args: WithdrawFromStagedPropArgs,
    ) -> Result<()> {
        ix::ix_withdraw_from_staged_prop(ctx, args)
    }

    // 3. Whenever funded, allow for 5 ix to be called that CPI into Futarchy Programs.
    // create_proposal_address, create_question_account, create_conditional_account, initialize_amms, split_tokens, add_liquidity, init_prop
    // 4. Set StagedProposal to MarketProposal

    // 5. Withdraw LP Rewards for LP
}
