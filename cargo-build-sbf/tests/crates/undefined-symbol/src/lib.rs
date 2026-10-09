//! Example Rust-based SBF program that calls an undefined symbol

use {
    solana_account_info::AccountInfo, solana_program_error::ProgramResult, solana_pubkey::Pubkey,
};

extern "C" {
    fn sol_undefined_symbol();
}

solana_program_entrypoint::entrypoint!(process_instruction);
fn process_instruction(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    unsafe { sol_undefined_symbol() };
    Ok(())
}
