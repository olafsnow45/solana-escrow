// use core::num;

use solana_program::program_error::ProgramError;

use crate::error::EscrowError::InvalidInstruction;

pub enum EscrowInstruction {
    /// Starts the trade by creating and populating an escrow account and transferring ownership of the given temp token account to the PDA
    ///
    ///
    /// Accounts expected:
    ///
    /// 0. `[signer]` The account of the person initializing the escrow
    /// 1. `[writable]` Temporary token account that should be created prior to this instruction and owned by the initializer
    /// 2. `[]` The initializer's token account for the token they will receive should the trade go through
    /// 3. `[writable]` The escrow account, it will hold all necessary info about the trade.
    /// 4. `[]` The rent sysvar
    /// 5. `[]` The token program
    InitEscrow {
        /// The amount party A expects to receive of token Y
        lamports: u64,
        sol_dir : u8,
        amount_x : u8,
        amount_y: u8,
        lamports_x: [u64; 9],
        lamports_y: [u64; 9]
    },
    /// Starts the trade by creating and populating an escrow account and transferring ownership of the given temp token account to the PDA
    ///
    ///
    /// Accounts expected:
    ///
    /// 0. `[signer]` The account of the person initializing the escrow
    /// 1. `[writable]` Temporary token account that should be created prior to this instruction and owned by the initializer
    /// 2. `[]` The initializer's token account for the token they will receive should the trade go through
    /// 3. `[writable]` The escrow account, it will hold all necessary info about the trade.
    /// 4. `[]` The rent sysvar
    /// 5. `[]` The token program
    CancelEscrow {
        /// The amount party A expects to receive of token Y
        lamports: u64,
        sol_dir : u8,
        amount_x : u8,
        amount_y: u8,
        lamports_x: [u64; 9],
        lamports_y: [u64; 9]
    },
    /// Accepts a trade
    ///
    ///
    /// Accounts expected:
    ///
    /// 0. `[signer]` The account of the person taking the trade
    /// 1. `[writable]` The taker's token account for the token they send
    /// 2. `[writable]` The taker's token account for the token they will receive should the trade go through
    /// 3. `[writable]` The PDA's temp token account to get tokens from and eventually close
    /// 4. `[writable]` The initializer's main account to send their rent fees to
    /// 5. `[writable]` The initializer's token account that will receive tokens
    /// 6. `[writable]` The escrow account holding the escrow info
    /// 7. `[]` The token program
    /// 8. `[]` The PDA account
    Exchange {
        /// the amount the taker expects to be paid in the other token, as a u64 because that's the max possible supply of a token
        lamports: u64,
        sol_dir : u8,
        amount_x : u8,
        amount_y: u8,
        lamports_x: [u64; 9],
        lamports_y: [u64; 9]
    },
}

impl EscrowInstruction {
    /// Unpacks a byte buffer into a [EscrowInstruction](enum.EscrowInstruction.html).
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {

        // let (tag, rest1) = input.split_first().ok_or(InvalidInstruction)?;
        // let (num_x, rest2) = rest1.split_first().ok_or(InvalidInstruction)?;
        // let (num_y, rest3) = rest2.split_first().ok_or(InvalidInstruction)?;
        // let (sol_dir, rest4) = rest3.split_first().ok_or(InvalidInstruction)?;
        // let lamports = Self::unpack_amount(rest4)?;

        let tag = &input[0];

        let sol_dir = &input[1];
        let lamports = Self::unpack_amount(&input[2..=9])?;

        let num_x = &input[10];

        let mut lamp_x: [u64; 9] = [0; 9];
        let mut lamp_y: [u64; 9] = [0; 9];

        for i in 0..(*num_x) {
            let i = i as usize;
            lamp_x[i] = Self::unpack_amount(&input[(11+i*8)..=(18+i*8)])?;
        }

        let num_x_usize = (*num_x) as usize;

        let num_y = &input[11+8*num_x_usize];
        for j in 0..(*num_y) {
            let j= j as usize;
            lamp_y[j] = Self::unpack_amount(&input[(12+8*(num_x_usize+j))..=(19+8*(num_x_usize+j))])?;
        }
        
        Ok(match tag {
            0 => Self::InitEscrow {
                // amount: Self::unpack_amount(rest)?,
                lamports: lamports,
                sol_dir: *sol_dir,
                amount_x: *num_x,
                amount_y: *num_y,
                lamports_x: lamp_x,
                lamports_y: lamp_y
            },
            1 => Self::Exchange {
                // amount: Self::unpack_amount(rest)?,
                lamports: lamports,
                sol_dir: *sol_dir,
                amount_x: *num_x,
                amount_y: *num_y,
                lamports_x: lamp_x,
                lamports_y: lamp_y
            },
            2 => Self::CancelEscrow {
                // amount: Self::unpack_amount(rest)?,
                lamports: lamports,
                sol_dir: *sol_dir,
                amount_x: *num_x,
                amount_y: *num_y,
                lamports_x: lamp_x,
                lamports_y: lamp_y
            },
            _ => return Err(InvalidInstruction.into()),
        })
    }

    fn unpack_amount(input: &[u8]) -> Result<u64, ProgramError> {
        let amount = input
            .get(..8)
            .and_then(|slice| slice.try_into().ok())
            .map(u64::from_le_bytes)
            .ok_or(InvalidInstruction)?;
        Ok(amount)
    }
}