use anchor_lang::prelude::*;
use anchor_spl::token::{transfer_checked, Mint, TokenAccount};

use crate::{errors::ErrorCode, PropLP, StagedProposal, StagedProposalState};

pub fn ix_fund_staged_prop(ctx: Context<FundStagedProp>, args: FundStagedPropArgs) -> Result<()> {
    if ctx.accounts.staged_prop.stage == StagedProposalState::Market {
        return Err(ErrorCode::StagedPropAlreadyMarket.into());
    }

    // If LP is being init, set lp_owner to payer
    ctx.accounts.lp_account.lp_owner = ctx.accounts.lp.key();
    ctx.accounts.lp_account.staged_prop = ctx.accounts.staged_prop.key();
    ctx.accounts.lp_account.rewards_redeemed = false;

    let staged_prop_token_amount = ctx.accounts.staged_prop_ata.amount;

    // Transfer tokens up to the amount in threshold
    if ctx.accounts.token_mint.key() == ctx.accounts.staged_prop.dao_token_thresholds.base_mint {
        let max_amount =
            ctx.accounts.staged_prop.dao_token_thresholds.base_threshold - staged_prop_token_amount;
        let amount_to_transfer = args.amount.min(max_amount);

        transfer_checked(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                anchor_spl::token::TransferChecked {
                    from: ctx.accounts.lp_token_ata.to_account_info(),
                    mint: ctx.accounts.token_mint.to_account_info(),
                    to: ctx.accounts.staged_prop_ata.to_account_info(),
                    authority: ctx.accounts.lp.to_account_info(),
                },
            ),
            amount_to_transfer,
            ctx.accounts.staged_prop.dao_token_thresholds.base_decimals,
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

        transfer_checked(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                anchor_spl::token::TransferChecked {
                    from: ctx.accounts.lp_token_ata.to_account_info(),
                    mint: ctx.accounts.token_mint.to_account_info(),
                    to: ctx.accounts.staged_prop_ata.to_account_info(),
                    authority: ctx.accounts.lp.to_account_info(),
                },
            ),
            amount_to_transfer,
            ctx.accounts.staged_prop.dao_token_thresholds.quote_decimals,
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

    pub token_mint: Account<'info, Mint>,
    pub lp_token_ata: Account<'info, TokenAccount>,
    pub staged_prop_ata: Account<'info, TokenAccount>,

    #[account(address = anchor_spl::token::ID)]
    pub token_program: AccountInfo<'info>,
}

pub fn ix_withdraw_from_staged_prop(
    ctx: Context<WithdrawFromStagedProp>,
    args: WithdrawFromStagedPropArgs,
) -> Result<()> {
    // We don't need to check if they are trying to withdraw during market stage because the staged prop ata simply won't have any tokens in it
    // Staged (lp -> staged_prop), Market (staged_prop -> prop), Finalized (prop -> staged_prop)

    let staged_id = ctx.accounts.staged_prop.staged_id;
    let signer_seeds = &[
        b"staged_prop",
        ctx.accounts.staged_prop.dao_address.as_ref(),
        &staged_id.to_le_bytes()[..],
        &[ctx.bumps.staged_prop],
    ];
    if ctx.accounts.token_mint.key() == ctx.accounts.staged_prop.dao_token_thresholds.base_mint {
        let max_amount = ctx.accounts.lp_account.base_token_contributed;
        let amount_to_transfer;
        if args.amount > max_amount {
            amount_to_transfer = max_amount;
        } else {
            amount_to_transfer = args.amount;
        }
        transfer_checked(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                anchor_spl::token::TransferChecked {
                    from: ctx.accounts.staged_prop_ata.to_account_info(),
                    mint: ctx.accounts.token_mint.to_account_info(),
                    to: ctx.accounts.lp_token_ata.to_account_info(),
                    authority: ctx.accounts.lp.to_account_info(),
                },
                &[signer_seeds],
            ),
            amount_to_transfer,
            ctx.accounts.staged_prop.dao_token_thresholds.base_decimals,
        )?;

        // TODO: Get treggs to check if this math is right
        if ctx.accounts.lp_account.rewards_redeemed == false {
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

        transfer_checked(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                anchor_spl::token::TransferChecked {
                    from: ctx.accounts.staged_prop_ata.to_account_info(),
                    mint: ctx.accounts.token_mint.to_account_info(),
                    to: ctx.accounts.lp_token_ata.to_account_info(),
                    authority: ctx.accounts.staged_prop.to_account_info(),
                },
                &[signer_seeds],
            ),
            amount_to_transfer,
            ctx.accounts.staged_prop.dao_token_thresholds.quote_decimals,
        )?;

        // TODO: Get treggs to check if this math is right
        if ctx.accounts.lp_account.rewards_redeemed == false {
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

    pub token_mint: Account<'info, Mint>,
    pub lp_token_ata: Account<'info, TokenAccount>,
    pub staged_prop_ata: Account<'info, TokenAccount>,

    #[account(address = anchor_spl::token::ID)]
    pub token_program: AccountInfo<'info>,
}

pub fn ix_withdraw_lp_rewards(ctx: Context<WithdrawLpRewards>) -> Result<()> {
    // Withdraw rewards if prop in market stage or later
    if ctx.accounts.staged_prop.stage == StagedProposalState::Staged {
        // Error: Cannot withdraw rewards if prop is in staged stage
        return Err(ErrorCode::StagedPropStillFunding.into());
    }

    let signer_seeds = &[
        b"staged_prop",
        ctx.accounts.staged_prop.dao_address.as_ref(),
        &ctx.accounts.staged_prop.staged_id.to_le_bytes()[..],
        &[ctx.bumps.staged_prop],
    ];

    // Withdraw rewards
    // Transfer Base Tokens from Prop LP to LP
    let base_token_reward = ctx
        .accounts
        .lp_account
        .base_token_contributed
        .checked_div(ctx.accounts.staged_prop.dao_token_thresholds.base_threshold)
        .unwrap()
        .checked_mul(
            ctx.accounts
                .staged_prop
                .reward_tokens
                .reward_token_amount_base,
        )
        .unwrap();

    transfer_checked(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            anchor_spl::token::TransferChecked {
                from: ctx.accounts.staged_prop_base_token_ata.to_account_info(),
                mint: ctx.accounts.token_mint.to_account_info(),
                to: ctx.accounts.lp_token_base_ata.to_account_info(),
                authority: ctx.accounts.staged_prop.to_account_info(),
            },
            &[signer_seeds],
        ),
        base_token_reward,
        ctx.accounts.staged_prop.dao_token_thresholds.base_decimals,
    )?;

    // Transfer Quote Tokens from Prop LP to LP
    let quote_token_reward = ctx
        .accounts
        .lp_account
        .quote_token_contributed
        .checked_div(
            ctx.accounts
                .staged_prop
                .dao_token_thresholds
                .quote_threshold,
        )
        .unwrap()
        .checked_mul(
            ctx.accounts
                .staged_prop
                .reward_tokens
                .reward_token_amount_quote,
        )
        .unwrap();

    transfer_checked(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            anchor_spl::token::TransferChecked {
                from: ctx.accounts.staged_prop_quote_token_ata.to_account_info(),
                mint: ctx.accounts.token_mint.to_account_info(),
                to: ctx.accounts.lp_token_quote_ata.to_account_info(),
                authority: ctx.accounts.staged_prop.to_account_info(),
            },
            &[signer_seeds],
        ),
        quote_token_reward,
        ctx.accounts.staged_prop.dao_token_thresholds.quote_decimals,
    )?;

    ctx.accounts.lp_account.rewards_redeemed = true;
    Ok(())
}

#[derive(Accounts)]
pub struct WithdrawLpRewards<'info> {
    #[account(mut)]
    pub lp: Signer<'info>,
    pub system_program: Program<'info, System>,

    #[account(
        seeds = [b"staged_prop", staged_prop.dao_address.as_ref(), staged_prop.staged_id.to_le_bytes().as_ref()],
        bump
    )]
    pub staged_prop: Account<'info, StagedProposal>,
    #[account(
        constraint = lp_account.lp_owner == lp.key()
    )]
    pub lp_account: Account<'info, PropLP>,

    pub token_mint: Account<'info, Mint>,
    pub lp_token_base_ata: Account<'info, TokenAccount>,
    pub lp_token_quote_ata: Account<'info, TokenAccount>,
    pub staged_prop_base_token_ata: Account<'info, TokenAccount>,
    pub staged_prop_quote_token_ata: Account<'info, TokenAccount>,
    #[account(address = anchor_spl::token::ID)]
    pub token_program: AccountInfo<'info>,
}

// TODO: Withdraw LP Fees
