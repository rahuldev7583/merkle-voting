use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Custom error message")]
    CustomError,
    #[msg("Merkle Proof doesn't match the stored root")]
    InvalidProof,
}
