use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::pubkey::Pubkey;

use crate::generated::state::{
	Account,
	AccountPDA,
	GemMetadata,
};


/// Accounts:
/// 0. `[writable, signer]` fee_payer: [AccountInfo] Auto-generated, default fee payer
/// 1. `[]` mint: [Mint] 
/// 2. `[writable]` gem: [GemMetadata] 
pub fn freeze(
	program_id: &Pubkey,
	mint: &Account<spl_token::state::Mint>,
	gem: &mut AccountPDA<GemMetadata>,
) -> ProgramResult {
    gem.data.freezed = true;
    Ok(())
}