use crate::state::StagedProposal;
use anchor_lang::{prelude::*, solana_program::native_token::LAMPORTS_PER_SOL};
use anchor_spl::token::{transfer_checked, TokenAccount};
use anchor_lang::solana_program::system_instruction::transfer;

use crate::state::*;

pub fn ix_create_staged_prop(ctx: Context<CreateStagedProp>, args: CreateStagedPropArgs) -> Result<()> {
    // Initialize Staged Proposal
    let staged_proposal: StagedProposal = StagedProposal {
        staged_id: args.staged_id,
        dao_address: args.dao_address,
        proposer: ctx.accounts.proposer.key(),
        description_url: args.description_url,
        instruction: args.instruction,
        stage: StagedProposalState::Staged,
        reward_tokens: args.reward_tokens.clone(),
        dao_token_thresholds: args.dao_token_thresholds,
    };
    ctx.accounts.staged_prop.set_inner(staged_proposal);

    // Fund the Stage Wallet with 1 SOL to make accounts. Can Reduce this maybe? Dunno how much we will need for AMMs and other things.
    // Refund after finalize
    transfer(
        &ctx.accounts.proposer.key(),
        &ctx.accounts.staged_wallet.key(),
        1*LAMPORTS_PER_SOL,
    );

    // Transfer Base Tokens from Proposer ATAs to Reward ATAs
    transfer_checked(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            anchor_spl::token::TransferChecked {
                from: ctx.accounts.proposer_base_token_ata.to_account_info(),
                mint: ctx.accounts.base_mint.to_account_info(),
                to: ctx.accounts.reward_token_base_ata.to_account_info(),
                authority: ctx.accounts.proposer.to_account_info(),
            },
        ),
        args.reward_tokens.reward_token_amount_base,
        args.reward_tokens.reward_token_decimals,
    )?;

    // Transfer Quote Tokens from Proposer ATAs to Reward ATAs
    transfer_checked(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            anchor_spl::token::TransferChecked {
                from: ctx.accounts.proposer_quote_token_ata.to_account_info(),
                mint: ctx.accounts.quote_mint.to_account_info(),
                to: ctx.accounts.reward_token_quote_ata.to_account_info(),
                authority: ctx.accounts.proposer.to_account_info(),
            },
        ),
        args.reward_tokens.reward_token_amount_quote,
        args.reward_tokens.reward_token_decimals,
    )?;

    Ok(())
}
#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct CreateStagedPropArgs {
    pub staged_id: u64,
    pub dao_address: Pubkey,
    pub description_url: String,
    pub instruction: StagedProposalInstruction,
    pub reward_tokens: RewardTokens,
    pub dao_token_thresholds: DAOTokenThresholds,
}

#[derive(Accounts)]
#[instruction(args: CreateStagedPropArgs)]
pub struct CreateStagedProp<'info> {
    #[account(mut)]
    pub proposer: Signer<'info>,
    pub system_program: Program<'info, System>,

    #[account(
        init, 
        payer=proposer, 
        space=8 + StagedProposal::INIT_SPACE,
        seeds = [b"staged_prop", args.dao_address.as_ref(), args.staged_id.to_le_bytes().as_ref()],
        bump
    )]
    pub staged_prop: Account<'info, StagedProposal>,
    #[account(
        seeds = [b"staged_wallet", staged_prop.key().as_ref()],
        bump,
    )]
    /// CHECK: Used as proposer wallet instead of PDA
    pub staged_wallet: UncheckedAccount<'info>,
    // Reward ATAs that hold Reward Tokens
    /// CHECK: Deserialize as SPL Token Account
    pub reward_token_base_ata: AccountInfo<'info>,
    /// CHECK: Deserialize as SPL Token Account
    pub reward_token_quote_ata: AccountInfo<'info>,

    // Proposer ATAs that transfer Reward Tokens
    /// CHECK: Deserialize as SPL Token Account
    pub proposer_base_token_ata: AccountInfo<'info>,
    /// CHECK: Deserialize as SPL Token Account
    pub proposer_quote_token_ata: AccountInfo<'info>,

    /// CHECK: Deserialize as SPL Token Program
    #[account(address = anchor_spl::token::ID)]
    pub token_program: AccountInfo<'info>,

    #[account(address = args.dao_token_thresholds.base_mint)]
    pub base_mint: Account<'info, TokenAccount>,
    #[account(address = args.dao_token_thresholds.quote_mint)]
    pub quote_mint: Account<'info, TokenAccount>,
}
