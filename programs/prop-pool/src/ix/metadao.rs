use crate::errors::ErrorCode;
use crate::StagedProposal;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::hash::hash as sha256;
use anchor_spl::token::{Mint, TokenAccount};

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
    pub amm: AccountInfo<'info>,
    /// CHECK: Init via CPI
    #[account(mut)]
    pub lp_mint: AccountInfo<'info>,
    /// CHECK: Passed along via CPI, no need to deserialize
    pub base_mint: AccountInfo<'info>,
    /// CHECK: Passed along via CPI, no need to deserialize
    pub quote_mint: AccountInfo<'info>,
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
