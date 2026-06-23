pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use error::ErrorCode;
pub use instructions::*;
pub use state::*;

declare_id!("GSE5sgXpXQMCzTkHojpXinocCCuTJhzWMhSTQysEjsmu");

#[program]
pub mod merkle_tree_verifier {
    use solana_program::hash::hashv;

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize::handler(ctx)
    }

    pub fn set_root(ctx: Context<SetRoot>, root: [u8; 32]) -> Result<()> {
        ctx.accounts.state.root = root;
        msg!("Root set {:?}", root);
        Ok(())
    }

    pub fn verify_proof(
        ctx: Context<VerifyProof>,
        leaf: [u8; 32],
        proof: Vec<[u8; 32]>,
        index: u8,
    ) -> Result<()> {
        let root = ctx.accounts.state.root;

        let mut current = leaf;

        for (depth, sibling) in proof.iter().enumerate() {
            let is_left = ((index >> depth) & 1) == 0;

            let hash = if is_left {
                hashv(&[&current, sibling])
            } else {
                hashv(&[sibling, &current])
            };

            current = hash.to_bytes();
        }

        require!(root == current, ErrorCode::InvalidProof);

        msg!("Merkle Proof Verified");

        Ok(())
    }
}

#[derive(Accounts)]
pub struct SetRoot<'info> {
    #[account(mut)]
    admin: Signer<'info>,
    #[account(init, payer = admin, space = 8 + 32 , seeds =[b"root"], bump)]
    state: Account<'info, MerkleState>,
    system_program: Program<'info, System>,
}

#[account]
pub struct MerkleState {
    root: [u8; 32],
}

#[derive(Accounts)]
pub struct VerifyProof<'info> {
    #[account(seeds = [b"root"], bump)]
    state: Account<'info, MerkleState>,
}
