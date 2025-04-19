use crate::errors::ErrorCode;
use crate::{StagedProposal, StagedProposalState};
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
            oracle: args.proposal_pubkey,
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

    /// CHECK: Program
    pub conditional_vault: AccountInfo<'info>,
}

pub fn ix_initialize_conditional_vault(ctx: Context<InitializeConditionalVault>) -> Result<()> {
    conditional_vault::cpi::initialize_conditional_vault(CpiContext::new(
        ctx.accounts.conditional_vault.to_account_info(),
        conditional_vault::cpi::accounts::InitializeConditionalVault {
            question: ctx.accounts.question_account.to_account_info(),
            payer: ctx.accounts.authority.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            event_authority: ctx.accounts.authority.to_account_info(),
            program: ctx.accounts.conditional_vault.to_account_info(),
            vault: ctx.accounts.base_vault.to_account_info(),
            underlying_token_mint: ctx.accounts.dao_token_mint.to_account_info(),
            vault_underlying_token_account: ctx
                .accounts
                .vault_underlying_token_account
                .to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
            associated_token_program: ctx.accounts.associated_token_program.to_account_info(),
        },
    ))?;
    Ok(())
}

#[derive(Accounts)]
pub struct InitializeConditionalVault<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        constraint = staged_prop.proposer == authority.key()
    )]
    pub staged_prop: Account<'info, StagedProposal>,

    pub system_program: Program<'info, System>,
    #[account(address = anchor_spl::token::ID)]
    pub token_program: AccountInfo<'info>,
    #[account(address = anchor_spl::associated_token::ID)]
    pub associated_token_program: AccountInfo<'info>,

    // CHECK: Program
    #[account(mut)]
    pub conditional_vault: AccountInfo<'info>,
    /// CHECK: Init in previous step
    pub question_account: AccountInfo<'info>,
    /// CHECK: Init via CPI
    #[account(mut)]
    pub base_vault: AccountInfo<'info>,
    /// CHECK: should be initialized via getOrCreateTokenAccount, passed along via CPI, no need to deserialize
    pub vault_underlying_token_account: AccountInfo<'info>,
    /// CHECK: Passed along via CPI, no need to deserialize
    pub dao_token_mint: AccountInfo<'info>,
}

// Called TWICE, once for Pass, once for Fail
pub fn ix_initialize_amms(ctx: Context<InitializeAmm>) -> Result<()> {
    amm::cpi::create_amm(
        CpiContext::new(
            ctx.accounts.amm_program.to_account_info(),
            amm::cpi::accounts::CreateAmm {
                user: ctx.accounts.authority.to_account_info(),
                amm: ctx.accounts.amm.to_account_info(),
                lp_mint: ctx.accounts.lp_mint.to_account_info(),
                base_mint: ctx.accounts.base_mint.to_account_info(),
                quote_mint: ctx.accounts.quote_mint.to_account_info(),
                vault_ata_base: ctx.accounts.vault_ata_base.to_account_info(),
                vault_ata_quote: ctx.accounts.vault_ata_quote.to_account_info(),
                associated_token_program: ctx.accounts.associated_token_program.to_account_info(),
                token_program: ctx.accounts.token_program.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                event_authority: ctx.accounts.authority.to_account_info(),
                program: ctx.accounts.amm_program.to_account_info(),
            },
        ),
        amm::instructions::CreateAmmArgs {
            twap_initial_observation: ctx.accounts.dao.twap_initial_observation,
            twap_max_observation_change_per_update: ctx
                .accounts
                .dao
                .twap_max_observation_change_per_update,
            twap_start_delay_slots: ctx.accounts.dao.twap_start_delay_slots,
        },
    )?;
    Ok(())
}

#[derive(Accounts)]
pub struct InitializeAmm<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
    #[account(
        constraint = staged_prop.proposer == authority.key()
    )]
    pub staged_prop: Account<'info, StagedProposal>,

    /// CHECK: Program
    pub amm_program: AccountInfo<'info>,
    /// CHECK: Init via CPI
    #[account(mut)]
    pub amm: AccountInfo<'info>, //different for pass/fail
    /// CHECK: Init via CPI
    #[account(mut)]
    pub lp_mint: AccountInfo<'info>,
    /// CHECK: Passed along via CPI, no need to deserialize
    pub base_mint: AccountInfo<'info>, //different for pass/fail
    /// CHECK: Passed along via CPI, no need to deserialize
    pub quote_mint: AccountInfo<'info>, //different for pass/fail
    /// CHECK: Init via CPI
    #[account(mut)]
    pub vault_ata_base: AccountInfo<'info>,
    /// CHECK: Init via CPI
    #[account(mut)]
    pub vault_ata_quote: AccountInfo<'info>,
    /// CHECK: Program
    #[account(address = anchor_spl::associated_token::ID)]
    pub associated_token_program: AccountInfo<'info>,
    /// CHECK: Program
    #[account(address = anchor_spl::token::ID)]
    pub token_program: AccountInfo<'info>,

    pub dao: Account<'info, autocrat::state::Dao>,
}

pub fn ix_split_tokens(ctx: Context<SplitTokens>) -> Result<()> {
    conditional_vault::cpi::split_tokens(
        CpiContext::new(
            ctx.accounts.conditional_vault.to_account_info(),
            conditional_vault::cpi::accounts::InteractWithVault {
                question: ctx.accounts.question_account.to_account_info(),
                vault: ctx.accounts.vault.to_account_info(),
                vault_underlying_token_account: ctx
                    .accounts
                    .vault_underlying_token_account
                    .to_account_info(),
                authority: ctx.accounts.authority.to_account_info(),
                user_underlying_token_account: ctx
                    .accounts
                    .user_underlying_token_account
                    .to_account_info(),
                token_program: ctx.accounts.token_program.to_account_info(),
                event_authority: ctx.accounts.authority.to_account_info(),
                program: ctx.accounts.conditional_vault.to_account_info(),
            },
        ),
        0,
    )?;
    Ok(())
}

#[derive(Accounts)]
pub struct SplitTokens<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,

    // Program
    pub conditional_vault: AccountInfo<'info>,
    /// CHECK: Passed along via CPI, no need to deserialize
    pub question_account: AccountInfo<'info>,
    /// CHECK: Passed along via CPI, no need to deserialize
    #[account(mut)]
    pub vault: AccountInfo<'info>,
    /// CHECK: Passed along via CPI, no need to deserialize
    pub vault_underlying_token_account: AccountInfo<'info>,
    /// CHECK: Passed along via CPI, no need to deserialize
    pub user_underlying_token_account: AccountInfo<'info>,
    /// CHECK: Program
    #[account(address = anchor_spl::token::ID)]
    pub token_program: AccountInfo<'info>,
}

// Called TWICE, once for Pass, once for Fail
pub fn ix_add_liquidity(ctx: Context<AddLiquidity>) -> Result<()> {
    amm::cpi::add_liquidity(
        CpiContext::new(
            ctx.accounts.amm.to_account_info(),
            amm::cpi::accounts::AddOrRemoveLiquidity {
                user: ctx.accounts.authority.to_account_info(),
                amm: ctx.accounts.amm.to_account_info(),
                lp_mint: ctx.accounts.lp_mint.to_account_info(),
                user_lp_account: ctx.accounts.user_lp_account.to_account_info(),
                user_base_account: ctx.accounts.user_base_account.to_account_info(),
                user_quote_account: ctx.accounts.user_quote_account.to_account_info(),
                vault_ata_base: ctx.accounts.vault_ata_base.to_account_info(),
                vault_ata_quote: ctx.accounts.vault_ata_quote.to_account_info(),
                token_program: ctx.accounts.token_program.to_account_info(),
                program: ctx.accounts.amm.to_account_info(),
                event_authority: ctx.accounts.authority.to_account_info(),
            },
        ),
        amm::instructions::AddLiquidityArgs {
            quote_amount: ctx
                .accounts
                .staged_prop
                .dao_token_thresholds
                .quote_threshold,
            max_base_amount: ctx.accounts.staged_prop.dao_token_thresholds.base_threshold,
            min_lp_tokens: 0, //Ask prophet what this amount should be.
        },
    )?;
    Ok(())
}

#[derive(Accounts)]
pub struct AddLiquidity<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,

    #[account(
        constraint = staged_prop.proposer == authority.key()
    )]
    pub staged_prop: Account<'info, StagedProposal>,

    /// CHECK: Program
    pub amm: AccountInfo<'info>,
    /// CHECK: Init via CPI
    pub lp_mint: AccountInfo<'info>,
    /// CHECK: Passed along via CPI, no need to deserialize
    pub user_lp_account: AccountInfo<'info>,
    /// CHECK: Passed along via CPI, no need to deserialize
    pub user_base_account: AccountInfo<'info>,
    /// CHECK: Passed along via CPI, no need to deserialize
    pub user_quote_account: AccountInfo<'info>,
    /// CHECK: Init via CPI
    pub vault_ata_base: AccountInfo<'info>,
    pub vault_ata_quote: AccountInfo<'info>,
    pub token_program: AccountInfo<'info>,
}

pub fn ix_init_proposal(ctx: Context<InitProposal>, args: IxInitProposalArgs) -> Result<()> {
    autocrat::cpi::initialize_proposal(
        CpiContext::new(
            ctx.accounts.autocrat.to_account_info(),
            autocrat::cpi::accounts::InitializeProposal {
                proposal: ctx.accounts.proposal.to_account_info(),
                dao: ctx.accounts.dao.to_account_info(),
                question: ctx.accounts.question.to_account_info(),
                quote_vault: ctx.accounts.quote_vault.to_account_info(),
                base_vault: ctx.accounts.base_vault.to_account_info(),
                pass_amm: ctx.accounts.pass_amm.to_account_info(),
                fail_amm: ctx.accounts.fail_amm.to_account_info(),
                pass_lp_mint: ctx.accounts.pass_lp_mint.to_account_info(),
                fail_lp_mint: ctx.accounts.fail_lp_mint.to_account_info(),
                pass_lp_user_account: ctx.accounts.pass_lp_user_account.to_account_info(),
                fail_lp_user_account: ctx.accounts.fail_lp_user_account.to_account_info(),
                pass_lp_vault_account: ctx.accounts.pass_lp_vault_account.to_account_info(),
                fail_lp_vault_account: ctx.accounts.fail_lp_vault_account.to_account_info(),
                token_program: ctx.accounts.token_program.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                event_authority: ctx.accounts.authority.to_account_info(),
                program: ctx.accounts.autocrat.to_account_info(),
                proposer: ctx.accounts.authority.to_account_info(),
            },
        ),
        autocrat::InitializeProposalParams {
            description_url: ctx.accounts.staged_prop.description_url.clone(),
            instruction: autocrat::ProposalInstruction {
                program_id: ctx.accounts.staged_prop.instruction.program_id,
                accounts: ctx
                    .accounts
                    .staged_prop
                    .instruction
                    .accounts
                    .iter()
                    .map(|a| autocrat::ProposalAccount {
                        pubkey: a.pubkey,
                        is_signer: a.is_signer,
                        is_writable: a.is_writable,
                    })
                    .collect(),
                data: ctx.accounts.staged_prop.instruction.data.clone(),
            },
            pass_lp_tokens_to_lock: ctx
                .accounts
                .staged_prop
                .dao_token_thresholds
                .quote_threshold,
            fail_lp_tokens_to_lock: ctx.accounts.staged_prop.dao_token_thresholds.base_threshold,
            nonce: args.nonce,
        },
    )?;
    Ok(())
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct IxInitProposalArgs {
    pub nonce: u64,
}

#[derive(Accounts)]
pub struct InitProposal<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,

    pub staged_prop: Account<'info, StagedProposal>,

    /// CHECK: Program
    pub autocrat: AccountInfo<'info>,
    /// CHECK: Init via CPI
    #[account(mut)]
    pub proposal: AccountInfo<'info>,
    /// CHECK: Passed through CPI
    #[account(mut)]
    pub dao: AccountInfo<'info>,
    /// CHECK: Passed through CPI
    pub question: AccountInfo<'info>,
    /// CHECK: Passed through CPI
    pub quote_vault: AccountInfo<'info>,
    /// CHECK: Passed through CPI
    pub base_vault: AccountInfo<'info>,
    /// CHECK: Passed through CPI
    pub pass_amm: AccountInfo<'info>,
    /// CHECK: Passed through CPI
    pub fail_amm: AccountInfo<'info>,
    /// CHECK: Passed through CPI
    pub pass_lp_mint: AccountInfo<'info>,
    /// CHECK: Passed through CPI
    pub fail_lp_mint: AccountInfo<'info>,
    /// CHECK: Passed through CPI
    #[account(mut)]
    pub pass_lp_user_account: AccountInfo<'info>,
    /// CHECK: Passed through CPI
    #[account(mut)]
    pub fail_lp_user_account: AccountInfo<'info>,
    /// CHECK: Passed through CPI
    #[account(mut)]
    pub pass_lp_vault_account: AccountInfo<'info>,
    /// CHECK: Passed through CPI
    #[account(mut)]
    pub fail_lp_vault_account: AccountInfo<'info>,
    /// CHECK: Program
    pub token_program: AccountInfo<'info>,
}

pub fn ix_execute_proposal(ctx: Context<ExecuteProposal>) -> Result<()> {
    autocrat::cpi::execute_proposal(CpiContext::new(
        ctx.accounts.autocrat.to_account_info(),
        autocrat::cpi::accounts::ExecuteProposal {
            proposal: ctx.accounts.proposal.to_account_info(),
            dao: ctx.accounts.dao.to_account_info(),
            event_authority: ctx.accounts.authority.to_account_info(),
            program: ctx.accounts.autocrat.to_account_info(),
        },
    ))?;

    ctx.accounts.staged_prop.stage = StagedProposalState::Executed;
    Ok(())
}

#[derive(Accounts)]
pub struct ExecuteProposal<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,

    #[account(
        mut,
        constraint = staged_prop.proposer == authority.key(),
        constraint = staged_prop.stage == StagedProposalState::Passed
    )]
    pub staged_prop: Account<'info, StagedProposal>,

    /// CHECK: Program
    pub autocrat: AccountInfo<'info>,
    /// CHECK: Passed through CPI
    #[account(mut)]
    pub proposal: AccountInfo<'info>,
    /// CHECK: Passed through CPI   
    pub dao: AccountInfo<'info>,
}

pub fn ix_finalize_proposal(ctx: Context<FinalizeProposal>) -> Result<()> {
    autocrat::cpi::finalize_proposal(CpiContext::new(
        ctx.accounts.autocrat.to_account_info(),
        autocrat::cpi::accounts::FinalizeProposal {
            proposal: ctx.accounts.proposal.to_account_info(),
            dao: ctx.accounts.dao.to_account_info(),
            question: ctx.accounts.question.to_account_info(),
            pass_amm: ctx.accounts.pass_amm.to_account_info(),
            fail_amm: ctx.accounts.fail_amm.to_account_info(),
            pass_lp_user_account: ctx.accounts.pass_lp_user_account.to_account_info(),
            fail_lp_user_account: ctx.accounts.fail_lp_user_account.to_account_info(),
            pass_lp_vault_account: ctx.accounts.pass_lp_vault_account.to_account_info(),
            fail_lp_vault_account: ctx.accounts.fail_lp_vault_account.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
            vault_program: ctx.accounts.vault_program.to_account_info(),
            vault_event_authority: ctx.accounts.vault_event_authority.to_account_info(),
            event_authority: ctx.accounts.authority.to_account_info(),
            program: ctx.accounts.autocrat.to_account_info(),
            treasury: ctx.accounts.treasury.to_account_info(),
        },
    ))?;

    ctx.accounts.staged_prop.stage = StagedProposalState::Finalized;
    Ok(())
}

#[derive(Accounts)]
pub struct FinalizeProposal<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,

    #[account(
        constraint = staged_prop.proposer == authority.key(),
        constraint = staged_prop.stage == StagedProposalState::Passed
    )]
    pub staged_prop: Account<'info, StagedProposal>,

    /// CHECK: Program
    pub autocrat: AccountInfo<'info>,
    /// CHECK: Passed through CPI
    #[account(mut)]
    pub proposal: AccountInfo<'info>,
    /// CHECK: Passed through CPI
    pub dao: AccountInfo<'info>,
    /// CHECK: Passed through CPI
    #[account(mut)]
    pub question: AccountInfo<'info>,
    /// CHECK: Passed through CPI
    pub pass_amm: AccountInfo<'info>,
    /// CHECK: Passed through CPI
    pub fail_amm: AccountInfo<'info>,
    /// CHECK: Passed through CPI
    #[account(mut)]
    pub pass_lp_user_account: AccountInfo<'info>,
    /// CHECK: Passed through CPI
    #[account(mut)]
    pub fail_lp_user_account: AccountInfo<'info>,
    /// CHECK: Passed through CPI
    #[account(mut)]
    pub pass_lp_vault_account: AccountInfo<'info>,
    /// CHECK: Passed through CPI
    #[account(mut)]
    pub fail_lp_vault_account: AccountInfo<'info>,
    /// CHECK: Program
    pub token_program: AccountInfo<'info>,
    /// CHECK: Program
    pub vault_program: AccountInfo<'info>,
    /// CHECK: Passed through CPI
    pub vault_event_authority: AccountInfo<'info>,
    /// CHECK: Passed through CPI
    pub treasury: AccountInfo<'info>,
}
