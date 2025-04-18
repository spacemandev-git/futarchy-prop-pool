use crate::errors::ErrorCode;
use crate::StagedProposal;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::hash::hash as sha256;
use anchor_spl::token::TokenAccount;

pub fn ix_initalize_question(
    ctx: Context<InitializeQuestion>,
    args: IxInitializeQuestionArgs,
) -> Result<()> {
    // Check Staged Prop ATAs have sufficient liquidity

    if ctx.accounts.staged_prop_base_token_ata.amount
        < ctx.accounts.staged_prop.dao_token_thresholds.base_threshold
    {
        return Err(ErrorCode::StagedPropNotLiquid.into());
    }

    if ctx.accounts.staged_prop_quote_token_ata.amount
        < ctx
            .accounts
            .staged_prop
            .dao_token_thresholds
            .quote_threshold
    {
        return Err(ErrorCode::StagedPropNotLiquid.into());
    }

    // Create a question with SHA-256 hash of "Will [proposal_address] pass?/FAIL/PASS"
    let question_data = format!("Will {} pass?/FAIL/PASS", args.proposal_pubkey);
    let question_id = sha256(question_data.as_bytes()).to_bytes();

    conditional_vault::cpi::initialize_question(
        CpiContext::new(
            ctx.accounts.conditional_vault.to_account_info(),
            conditional_vault::cpi::accounts::InitializeQuestion {
                question: ctx.accounts.question_account.to_account_info(),
                payer: ctx.accounts.authority.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                event_authority: ctx.accounts.authority.to_account_info(),
                program: ctx.accounts.conditional_vault.to_account_info(),
            },
        ),
        conditional_vault::InitializeQuestionArgs {
            question_id,
            oracle: ctx.accounts.authority.key(),
            num_outcomes: 2,
        },
    )?;

    Ok(())
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct IxInitializeQuestionArgs {
    pub proposal_pubkey: Pubkey,
}

#[derive(Accounts)]
pub struct InitializeQuestion<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,

    #[account(
        constraint = staged_prop.proposer == authority.key()
    )]
    pub staged_prop: Account<'info, StagedProposal>,
    pub staged_prop_base_token_ata: Account<'info, TokenAccount>,
    pub staged_prop_quote_token_ata: Account<'info, TokenAccount>,

    /// CHECK: Init by CPI
    #[account(mut)]
    pub question_account: AccountInfo<'info>,

    pub conditional_vault: AccountInfo<'info>,
}
