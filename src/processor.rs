use std::str::FromStr;

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    clock::Clock,
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
    sysvar::{rent::Rent, Sysvar},
};

use crate::{
    error::StreamError,
    instruction::StreamInstruction,
    state::{CreateStreamState, StreamData, WithdrawStreamState},
};

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
        accounts: &[AccountInfo],
        data: CreateStreamState,
    ) -> ProgramResult {
        let admin_pub_key = match Pubkey::from_str("") {
            Ok(key) => key,
            Err(_) => return Err(StreamError::PubKeyParseError.into()),
        };

        let account_info_iter = &mut accounts.iter();

        let escrow_account = next_account_info(account_info_iter)?;
        let sender_account = next_account_info(account_info_iter)?;
        let receiver_account = next_account_info(account_info_iter)?;
        let admin_account = next_account_info(account_info_iter)?;

        if *admin_account.key != admin_pub_key {
            return Err(StreamError::AdminAccountInvalid.into());
        }

        **escrow_account.try_borrow_mut_lamports()? -= 30000000;
        **admin_account.try_borrow_mut_lamports()? += 30000000;

        if data.end_time <= data.start_time || data.start_time < Clock::get()?.unix_timestamp {
            return Err(StreamError::InvalidStartOrEndTime.into());
        }

        if data.amount_second * ((data.end_time - data.start_time) as u64)
            != **escrow_account.lamports.borrow()
                - Rent::get()?.minimum_balance(escrow_account.data_len())
        {
            return Err(StreamError::NotEnoughLamports.into());
        }

        if !sender_account.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        if *receiver_account.key != data.receiver {
            return Err(ProgramError::InvalidAccountData);
        }

        let escrow_data = StreamData::new(data, *sender_account.key);

        escrow_data.serialize(&mut &mut escrow_account.data.borrow_mut()[..])?;
        Ok(())
    }

    fn process_withdraw(
        _program_id: &Pubkey,
        accounts: &[AccountInfo],
        data: WithdrawStreamState,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let escrow_account = next_account_info(account_info_iter)?;
        let receiver_account = next_account_info(account_info_iter)?;

        let mut escrow_data = StreamData::try_from_slice(&escrow_account.data.borrow())
            .expect("Failed to deserialize escrow data");
        if *receiver_account.key != escrow_data.receiver {
            return Err(ProgramError::IllegalOwner);
        }

        if !receiver_account.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        let time = Clock::get()?.unix_timestamp;
        let withdrawable_amount = escrow_data.amount_second
            * (std::cmp::min(time, escrow_data.end_time) as u64)
            - escrow_data.lamports_withdraw;

        if data.amount > withdrawable_amount {
            return Err(StreamError::WithdrawError.into());
        }

        **escrow_account.try_borrow_mut_lamports()? -= data.amount;
        **receiver_account.try_borrow_mut_lamports()? += data.amount;
        escrow_data.lamports_withdraw += data.amount;

        escrow_data.serialize(&mut &mut escrow_account.data.borrow_mut()[..])?;
        Ok(())
    }

    fn process_close(_program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let escrow_account = next_account_info(account_info_iter)?;
        let sender_account = next_account_info(account_info_iter)?;
        let receiver_account = next_account_info(account_info_iter)?;

        let mut escrow_data = StreamData::try_from_slice(&escrow_account.data.borrow())
            .expect("Failed to serialize escrow data");

        if escrow_data.sender != *sender_account.key {
            return Err(ProgramError::IllegalOwner);
        }
        if !sender_account.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        let time: i64 = Clock::get()?.unix_timestamp;
        let mut lamport_streamed_to_receiver: u64 = 0;

        if time > escrow_data.start_time {
            lamport_streamed_to_receiver = escrow_data.amount_second
                * ((std::cmp::min(time, escrow_data.end_time) - escrow_data.start_time) as u64)
                - escrow_data.lamports_withdraw;
        }

        **receiver_account.try_borrow_mut_lamports()? += lamport_streamed_to_receiver;
        escrow_data.lamports_withdraw += lamport_streamed_to_receiver;
        **sender_account.try_borrow_mut_lamports()? += **escrow_account.lamports.borrow();

        **escrow_account.try_borrow_mut_lamports()? = 0;

        Ok(())
    }
}
