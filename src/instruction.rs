use crate::state::{CreateStreamState, WithdrawStreamState};
use borsh::BorshDeserialize;
use solana_program::program_error::ProgramError;

#[derive(Clone, Debug, PartialEq)]
pub enum StreamInstruction {
    /// Account expected:
    ///
    /// `[writable]` escrow account
    /// `[signer]` sender account
    /// `[]` receiver account
    /// `[]` admin account
    CreateStream(CreateStreamState),

    /// Account expected:
    ///
    /// `[writable]` escrow account
    /// `[signer]` receiver account
    WithdrawStream(WithdrawStreamState),

    /// Account expected:
    ///
    /// `[writable]` escrow account
    /// `[signer]` sender account
    /// `[]` receiver account
    CloseStream,
}

impl StreamInstruction {
    pub fn unpack(instruction_data: &[u8]) -> Result<Self, ProgramError> {
        let (tag, data) = instruction_data.split_first().unwrap();
        match tag {
            1 => Ok(StreamInstruction::CreateStream(
                CreateStreamState::try_from_slice(data)?,
            )),
            2 => Ok(StreamInstruction::WithdrawStream(
                WithdrawStreamState::try_from_slice(data)?,
            )),
            3 => Ok(StreamInstruction::CloseStream),
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}
