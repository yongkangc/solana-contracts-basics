
use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]

// an enum that defines all the possible instructions that can be sent to the program:
// 0 - Create a new token
// 1 - Create a new token account
// 2 - Mint some tokens to a token account
// 3 - Transfer tokens between token accounts
pub enum TokenInstruction {
   CreateToken,
   CreateTokenAccount,
   Mint { amount: u64 },
   Transfer { amount: u64 },
}