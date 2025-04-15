use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct StagedProposal {
    pub staged_id: u64,
    pub dao_address: Pubkey,
    pub proposer: Pubkey,
    #[max_len(255)]
    pub description_url: String,
    pub instruction: StagedProposalInstruction,
    pub stage: StagedProposalState,
    pub reward_tokens: RewardTokens,
    pub dao_token_thresholds: DAOTokenThresholds,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize, Debug, PartialEq, Eq, InitSpace)]
pub struct StagedProposalAccount {
    pub pubkey: Pubkey,
    pub is_signer: bool,
    pub is_writable: bool,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize, Debug, PartialEq, Eq, InitSpace)]
pub struct StagedProposalInstruction {
    pub program_id: Pubkey,
    #[max_len(12)]
    pub accounts: Vec<StagedProposalAccount>,
    #[max_len(600)]
    pub data: Vec<u8>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize, Debug, PartialEq, Eq, InitSpace)]
pub enum StagedProposalState {
    Staged,
    Market,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize, Debug, PartialEq, Eq, InitSpace)]
pub struct RewardTokens {
    pub reward_token_mint: Pubkey,
    pub reward_token_amount_base: u64, // total amount of tokens rewarded for LP'ing the base token
    pub reward_token_amount_quote: u64, // total amount of tokens rewarded for LP'ing the quote token
    pub reward_token_decimals: u8,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize, Debug, PartialEq, Eq, InitSpace)]
pub struct DAOTokenThresholds {
    pub base_mint: Pubkey,
    pub quote_mint: Pubkey,
    pub base_threshold: u64, // total amount of base tokens that must be locked in the LP for the proposal to go to market
    pub base_decimals: u8,
    pub quote_threshold: u64, // total amount of quote tokens that must be locked in the LP for the proposal to go to market
    pub quote_decimals: u8,
}
