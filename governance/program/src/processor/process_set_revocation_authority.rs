//! Program state processor

use borsh::BorshSerialize;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    pubkey::Pubkey,
};

use crate::state::token_owner_record::get_token_owner_record_data;

/// Processes SetGovernanceDelegate instruction
pub fn process_set_revocation_authority(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    new_revocation_authority: &Option<Pubkey>,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    let governance_authority_info = next_account_info(account_info_iter)?; // 0
    let token_owner_record_info = next_account_info(account_info_iter)?; // 1

    let mut token_owner_record_data =
        get_token_owner_record_data(program_id, token_owner_record_info)?;

    let permitted_account = match token_owner_record_info.revocation_authority {
        Some(auth) => Role::Revoker,
        None => Role::Depositor
    };

    token_owner_record_data.assert_permitted_account_is_signer(governance_authority_info, vec![permitted_account])?;

    token_owner_record_data.revocation_authority = *new_revocation_authority;
    token_owner_record_data.serialize(&mut *token_owner_record_info.data.borrow_mut())?;

    Ok(())
}
