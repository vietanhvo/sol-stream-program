use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey};

use crate::instruction::StreamInstruction;

pub struct Processor;

impl Processor {
    pub fn process(
        _program_id: &Pubkey,
        _account: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction = StreamInstruction::unpack(instruction_data)?;

        match instruction {
            StreamInstruction::CreateStream(_data) => todo!(),
            StreamInstruction::WithdrawStream(_data) => todo!(),
            StreamInstruction::CloseStream => todo!(),
        }
    }
    fn process_create(
        _program_id: &Pubkey,
        _account: &[AccountInfo],
        _instruction_data: &[u8],
    ) -> ProgramResult {
        Ok(())
    }

    fn process_withdraw(
        _program_id: &Pubkey,
        _account: &[AccountInfo],
        _instruction_data: &[u8],
    ) -> ProgramResult {
        Ok(())
    }

    fn process_close(_program_id: &Pubkey, _account: &[AccountInfo]) -> ProgramResult {
        Ok(())
    }
}
