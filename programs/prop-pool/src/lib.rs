#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;

declare_id!("3yrRGtxcxs2pRoBYPsz2DYkiQDQdxqjv1wyGMySNSZ15");

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

    // 3. Whenever funded, initialize a proposal in the Futarchy Program
    pub fn init_question(
        ctx: Context<InitializeQuestion>,
        args: IxInitializeQuestionArgs,
    ) -> Result<()> {
        ix::ix_initalize_question(ctx, args)
    }

    pub fn init_conditional_vault(ctx: Context<InitializeConditionalVault>) -> Result<()> {
        ix::ix_initialize_conditional_vault(ctx)
    }

    pub fn init_amms(ctx: Context<InitializeAmm>) -> Result<()> {
        ix::ix_initialize_amms(ctx)
    }

    pub fn split_tokens(ctx: Context<SplitTokens>) -> Result<()> {
        ix::ix_split_tokens(ctx)
    }

    pub fn add_liquidity(ctx: Context<AddLiquidity>) -> Result<()> {
        ix::ix_add_liquidity(ctx)
    }

    pub fn init_proposal(ctx: Context<InitProposal>, args: IxInitProposalArgs) -> Result<()> {
        ix::ix_init_proposal(ctx, args)
    }

    //4. Execute Proposal
    pub fn execute_proposal(ctx: Context<ExecuteProposal>) -> Result<()> {
        ix::ix_execute_proposal(ctx)
    }

    //5. Finalize Proposal
    pub fn finalize_proposal(ctx: Context<FinalizeProposal>) -> Result<()> {
        ix::ix_finalize_proposal(ctx)
    }

    //6. Withdraw LP Fees??
}
