use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{clock::UnixTimestamp, pubkey::Pubkey};

#[derive(Clone, Debug, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct CreateStreamState {
    pub start_time: UnixTimestamp,
    pub end_time: UnixTimestamp,
    pub receiver: Pubkey,
    pub lamports_withdraw: u64,
    pub amount_second: u64,
}

#[derive(Clone, Debug, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct WithdrawStreamState {
    pub amount: u64,
}

#[derive(Clone, Debug, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct StreamData {
    pub start_time: UnixTimestamp,
    pub end_time: UnixTimestamp,
    pub receiver: Pubkey,
    pub lamports_withdraw: u64,
    pub amount_second: u64,
    pub sender: Pubkey,
}

impl StreamData {
    pub fn new(data: CreateStreamState, sender: Pubkey) -> Self {
        StreamData {
            start_time: data.start_time,
            end_time: data.end_time,
            receiver: data.receiver,
            lamports_withdraw: 0,
            amount_second: data.amount_second,
            sender,
        }
    }
}

// Added this test to check the size of the StreamData
#[test]
fn check_size() {
    let stream = StreamData {
        start_time: 0,
        end_time: 0,
        receiver: Pubkey::new_unique(),
        sender: Pubkey::new_unique(),
        lamports_withdraw: 0,
        amount_second: 0,
    };
    let compress = stream.try_to_vec().expect("something");
    println!("{}", compress.len())
}
