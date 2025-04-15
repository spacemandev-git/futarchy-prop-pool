use anchor_lang::error_code;

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid token mint")]
    InvalidTokenMint,
    #[msg("Staged proposal already in market")]
    StagedPropAlreadyMarket,
}
