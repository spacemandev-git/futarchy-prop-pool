use anchor_lang::error_code;

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid token mint")]
    InvalidTokenMint,
    #[msg("Staged proposal already in market")]
    StagedPropAlreadyMarket,
    #[msg("Staged prop not liquid enough")]
    StagedPropNotLiquid,
    #[msg("Cannot withdraw rewards if prop is still funding")]
    StagedPropStillFunding,
}
