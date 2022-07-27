use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, pubkey::Pubkey,
};

use crate::processor::Processor;

fn process_instruction(
    program_id: &Pubkey,
    account: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    Processor::process(program_id, account, instruction_data)
}

entrypoint!(process_instruction);
