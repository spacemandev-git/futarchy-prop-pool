use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct PropLP {
    pub lp_owner: Pubkey,
    pub staged_prop: Pubkey,
    pub base_token_contributed: u64,
    pub quote_token_contributed: u64,
    pub base_token_reward_redeemed: u64,
    pub quote_token_reward_redeemed: u64,
}
