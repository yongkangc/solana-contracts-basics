// The code here defines the data that will be stored for the program, which is split across two structs
// 1. a header level ‘Token’ struct that defines header level information about the token
// 2. ‘TokenAccount’ struct, that defines information about an account for the specified token

// helper functions included for each struct to help the program retrieve and save data

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
   account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
   pubkey::Pubkey,
};

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct Token {
   pub authority: Pubkey,
   pub supply: u64,
}

impl Token {
   pub fn load_unchecked(ai: &AccountInfo) -> Result<Self, ProgramError> {
       Ok(Self::try_from_slice(&ai.data.borrow())?)
   }
   pub fn save(&self, ai: &AccountInfo) -> ProgramResult {
       Ok(self.serialize(&mut *ai.data.borrow_mut())?)
   }

   pub fn load(ai: &AccountInfo) -> Result<Self, ProgramError> {
       let token = Self::try_from_slice(&ai.data.borrow())?;
       Ok(token)
   }
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct TokenAccount {
   pub owner: Pubkey,
   pub token: Pubkey,
   pub amount: u64,
}

impl TokenAccount {
   pub fn load_unchecked(ai: &AccountInfo) -> Result<Self, ProgramError> {
       Ok(Self::try_from_slice(&ai.data.borrow())?)
   }
   pub fn save(&self, ai: &AccountInfo) -> ProgramResult {
       Ok(self.serialize(&mut *ai.data.borrow_mut())?)
   }

   pub fn load(ai: &AccountInfo) -> Result<Self, ProgramError> {
       let account = Self::try_from_slice(&ai.data.borrow())?;
       Ok(account)
   }
}