use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::{invoke, invoke_signed};
use spl_token::{instruction::transfer_checked, solana_program::program_pack::Pack};

use crate::{errors::ErrorCode, PropLP, StagedProposal, StagedProposalState};

pub fn ix_fund_staged_prop(ctx: Context<FundStagedProp>, args: FundStagedPropArgs) -> Result<()> {
    if ctx.accounts.staged_prop.stage == StagedProposalState::Market {
        return Err(ErrorCode::StagedPropAlreadyMarket.into());
    }

    let staged_prop_token_ata =
        spl_token::state::Account::unpack(&ctx.accounts.staged_prop_ata.data.borrow())?;

    let staged_prop_token_amount = staged_prop_token_ata.amount;

    // Transfer tokens up to the amount in threshold
    if ctx.accounts.token_mint.key() == ctx.accounts.staged_prop.dao_token_thresholds.base_mint {
        let max_amount =
            ctx.accounts.staged_prop.dao_token_thresholds.base_threshold - staged_prop_token_amount;
        let amount_to_transfer = args.amount.min(max_amount);

        let transfer_ix = transfer_checked(
            &ctx.accounts.token_program.key(),
            &ctx.accounts.lp_token_ata.key(),
            &ctx.accounts.token_mint.key(),
            &ctx.accounts.staged_prop_ata.key(),
            &ctx.accounts.lp.key(),
            &[&ctx.accounts.lp.key()],
            amount_to_transfer,
            ctx.accounts.staged_prop.dao_token_thresholds.base_decimals,
        )?;

        invoke(
            &transfer_ix,
            &[
                ctx.accounts.token_program.to_account_info(),
                ctx.accounts.lp_token_ata.to_account_info(),
                ctx.accounts.token_mint.to_account_info(),
                ctx.accounts.staged_prop_ata.to_account_info(),
                ctx.accounts.lp.to_account_info(),
            ],
        )?;

        ctx.accounts.lp_account.base_token_contributed += amount_to_transfer;
    } else if ctx.accounts.token_mint.key()
        == ctx.accounts.staged_prop.dao_token_thresholds.quote_mint
    {
        let max_amount = ctx
            .accounts
            .staged_prop
            .dao_token_thresholds
            .quote_threshold
            - staged_prop_token_amount;
        let amount_to_transfer = args.amount.min(max_amount);

        let transfer_ix = transfer_checked(
            &ctx.accounts.token_program.key(),
            &ctx.accounts.lp_token_ata.key(),
            &ctx.accounts.token_mint.key(),
            &ctx.accounts.staged_prop_ata.key(),
            &ctx.accounts.lp.key(),
            &[&ctx.accounts.lp.key()],
            amount_to_transfer,
            ctx.accounts.staged_prop.dao_token_thresholds.quote_decimals,
        )?;

        invoke(
            &transfer_ix,
            &[
                ctx.accounts.token_program.to_account_info(),
                ctx.accounts.lp_token_ata.to_account_info(),
                ctx.accounts.token_mint.to_account_info(),
                ctx.accounts.staged_prop_ata.to_account_info(),
                ctx.accounts.lp.to_account_info(),
            ],
        )?;

        ctx.accounts.lp_account.quote_token_contributed += amount_to_transfer;
    } else {
        return Err(ErrorCode::InvalidTokenMint.into());
    }

    Ok(())
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct FundStagedPropArgs {
    pub amount: u64,
}

#[derive(Accounts)]
pub struct FundStagedProp<'info> {
    #[account(mut)]
    pub lp: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub staged_prop: Account<'info, StagedProposal>,

    #[account(
        init_if_needed,
        payer = lp,
        space = 8 + PropLP::INIT_SPACE,
        seeds = [b"lp", lp.key().as_ref(), staged_prop.key().as_ref()],
        bump
    )]
    pub lp_account: Account<'info, PropLP>,

    pub token_mint: AccountInfo<'info>,
    pub lp_token_ata: AccountInfo<'info>,
    pub staged_prop_ata: AccountInfo<'info>,

    #[account(address = spl_token::ID)]
    pub token_program: AccountInfo<'info>,
}

pub fn ix_withdraw_from_staged_prop(
    ctx: Context<WithdrawFromStagedProp>,
    args: WithdrawFromStagedPropArgs,
) -> Result<()> {
    let staged_prop_token_ata =
        spl_token::state::Account::unpack(&ctx.accounts.staged_prop_ata.data.borrow())?;

    let staged_prop_token_amount = staged_prop_token_ata.amount;

    if ctx.accounts.token_mint.key() == ctx.accounts.staged_prop.dao_token_thresholds.base_mint {
        let max_amount = ctx.accounts.lp_account.base_token_contributed;
        let amount_to_transfer;
        if args.amount > max_amount {
            amount_to_transfer = max_amount;
        } else {
            amount_to_transfer = args.amount;
        }

        let transfer_ix = transfer_checked(
            &ctx.accounts.token_program.key(),
            &ctx.accounts.staged_prop_ata.key(),
            &ctx.accounts.token_mint.key(),
            &ctx.accounts.lp_token_ata.key(),
            &ctx.accounts.staged_prop.key(),
            &[&ctx.accounts.staged_prop.key()],
            amount_to_transfer,
            ctx.accounts.staged_prop.dao_token_thresholds.base_decimals,
        )?;

        let staged_id = ctx.accounts.staged_prop.staged_id;
        let signer_seeds = &[
            b"staged_prop",
            ctx.accounts.staged_prop.dao_address.as_ref(),
            &staged_id.to_le_bytes()[..],
            &[ctx.bumps.staged_prop],
        ];

        invoke_signed(
            &transfer_ix,
            &[
                ctx.accounts.token_program.to_account_info(),
                ctx.accounts.lp_token_ata.to_account_info(),
                ctx.accounts.token_mint.to_account_info(),
                ctx.accounts.staged_prop_ata.to_account_info(),
                ctx.accounts.lp.to_account_info(),
            ],
            &[signer_seeds],
        )?;

        // TODO: Get treggs to check if this math is right
        if ctx.accounts.lp_account.base_token_reward_redeemed == 0 {
            ctx.accounts.lp_account.base_token_contributed -= amount_to_transfer;
        }
    } else if ctx.accounts.token_mint.key()
        == ctx.accounts.staged_prop.dao_token_thresholds.quote_mint
    {
        let max_amount = ctx.accounts.lp_account.quote_token_contributed;
        let amount_to_transfer;
        if args.amount > max_amount {
            amount_to_transfer = max_amount;
        } else {
            amount_to_transfer = args.amount;
        }

        let transfer_ix = transfer_checked(
            &ctx.accounts.token_program.key(),
            &ctx.accounts.staged_prop_ata.key(),
            &ctx.accounts.token_mint.key(),
            &ctx.accounts.lp_token_ata.key(),
            &ctx.accounts.staged_prop.key(),
            &[&ctx.accounts.staged_prop.key()],
            amount_to_transfer,
            ctx.accounts.staged_prop.dao_token_thresholds.quote_decimals,
        )?;

        let staged_id = ctx.accounts.staged_prop.staged_id;
        let signer_seeds = &[
            b"staged_prop",
            ctx.accounts.staged_prop.dao_address.as_ref(),
            &staged_id.to_le_bytes()[..],
            &[ctx.bumps.staged_prop],
        ];

        invoke_signed(
            &transfer_ix,
            &[
                ctx.accounts.token_program.to_account_info(),
                ctx.accounts.lp_token_ata.to_account_info(),
                ctx.accounts.token_mint.to_account_info(),
                ctx.accounts.staged_prop_ata.to_account_info(),
                ctx.accounts.lp.to_account_info(),
            ],
            &[signer_seeds],
        )?;

        // TODO: Get treggs to check if this math is right
        if ctx.accounts.lp_account.quote_token_reward_redeemed == 0 {
            ctx.accounts.lp_account.quote_token_contributed -= amount_to_transfer;
        }
    } else {
        return Err(ErrorCode::InvalidTokenMint.into());
    }
    Ok(())
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct WithdrawFromStagedPropArgs {
    pub amount: u64,
}

#[derive(Accounts)]
pub struct WithdrawFromStagedProp<'info> {
    #[account(mut)]
    pub lp: Signer<'info>,
    pub system_program: Program<'info, System>,

    #[account(
        seeds = [b"staged_prop", staged_prop.dao_address.as_ref(), staged_prop.staged_id.to_le_bytes().as_ref()],
        bump
    )]
    pub staged_prop: Account<'info, StagedProposal>,

    #[account(
        seeds = [b"lp", lp.key().as_ref(), staged_prop.key().as_ref()],
        bump
    )]
    pub lp_account: Account<'info, PropLP>,

    pub token_mint: AccountInfo<'info>,
    pub lp_token_ata: AccountInfo<'info>,
    pub staged_prop_ata: AccountInfo<'info>,

    #[account(address = spl_token::ID)]
    pub token_program: AccountInfo<'info>,
}
