use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    pubkey::Pubkey,
    sysvar::{rent::Rent, Sysvar},
    system_instruction,
};

use crate::{error::EscrowError, instruction::EscrowInstruction};
use arrayref::{array_mut_ref, array_ref};

pub struct Processor;
impl Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        msg!("Process -> Instruction");
        let instruction = EscrowInstruction::unpack(instruction_data)?;

        msg!("Instruction -> Init");
        match instruction {
            EscrowInstruction::InitEscrow { lamports, sol_dir, amount_x, amount_y, lamports_x, lamports_y } => {
                msg!("Instruction: InitEscrow");
                Self::process_init_escrow(accounts, lamports, sol_dir, amount_x, amount_y, lamports_x, lamports_y, program_id)
            }
            EscrowInstruction::Exchange { lamports, sol_dir, amount_x, amount_y, lamports_x, lamports_y } => {
                msg!("Instruction: Exchange");
                Self::process_exchange(accounts, lamports, sol_dir, amount_x, amount_y, lamports_x, lamports_y, program_id)
            }
            EscrowInstruction::CancelEscrow { lamports, sol_dir, amount_x, amount_y, lamports_x, lamports_y } => {
                msg!("Instruction: CancelEscrow");
                Self::process_cancel_escrow(accounts, lamports, sol_dir, amount_x, amount_y, lamports_x, lamports_y, program_id)
            }
        }
    }

    fn process_init_escrow(
        accounts: &[AccountInfo],
        lamports: u64,
        sol_dir: u8,
        amount_x: u8,
        amount_y: u8,
        lamports_x: [u64; 9],
        lamports_y: [u64; 9],
        program_id: &Pubkey,
    ) -> ProgramResult {

        let account_info_iter = &mut accounts.iter();

        let initializer = next_account_info(account_info_iter)?;
        if !initializer.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        msg!("initializer Pubkey : {}", initializer.key);

        let taker_account = next_account_info(account_info_iter)?;
        msg!("Taker Pubkey : {}", taker_account.key);
        
        let escrow_account = next_account_info(account_info_iter)?;
        msg!("Escrow account Pubkey : {}", escrow_account.key );

        let rent = &Rent::from_account_info(next_account_info(account_info_iter)?)?;

        if !rent.is_exempt(escrow_account.lamports(), escrow_account.data_len()) {
            msg!("Rent error --------> ???");
            return Err(EscrowError::NotRentExempt.into());
        }

        let (pda, _nonce) = Pubkey::find_program_address(&[b"escrow"], program_id);
        let token_program = next_account_info(account_info_iter)?;
        msg!("token_program : {}", token_program.key);

        let mut initializer_token_account;
        let mut taker_token_account;
        let mut temp_token_account;

        let mut transfer_initializer_to_temp_ix;
        let mut owner_change_ix;

        msg!("escrow_account : {}", escrow_account.key);

        let mut escrow_data_pos = 0;
        {
            let escrow_mut_data = &mut escrow_account.try_borrow_mut_data()?;

            if escrow_mut_data[0] != 0 {
                msg!("escrow_account data exists already!!!");
                return Err(ProgramError::AccountAlreadyInitialized);
            }
            msg!("escrow_account -> OK");

            escrow_mut_data[escrow_data_pos] = 1;
            escrow_data_pos += 1;
            escrow_mut_data[escrow_data_pos] = amount_x as u8;
            escrow_data_pos += 1;
            escrow_mut_data[escrow_data_pos] = amount_y as u8;
            escrow_data_pos += 1;
            escrow_mut_data[escrow_data_pos] = sol_dir as u8;
            escrow_data_pos += 1;

            escrow_mut_data[escrow_data_pos] = (lamports >> 56) as u8;
            escrow_data_pos += 1;
            escrow_mut_data[escrow_data_pos] = (lamports >> 48) as u8;
            escrow_data_pos += 1;
            escrow_mut_data[escrow_data_pos] = (lamports >> 40) as u8;
            escrow_data_pos += 1;
            escrow_mut_data[escrow_data_pos] = (lamports >> 32) as u8;
            escrow_data_pos += 1;
            escrow_mut_data[escrow_data_pos] = (lamports >> 24) as u8;
            escrow_data_pos += 1;
            escrow_mut_data[escrow_data_pos] = (lamports >> 16) as u8;
            escrow_data_pos += 1;
            escrow_mut_data[escrow_data_pos] = (lamports >> 8) as u8;
            escrow_data_pos += 1;
            escrow_mut_data[escrow_data_pos] = lamports as u8;
            escrow_data_pos += 1;

            array_mut_ref![escrow_mut_data, escrow_data_pos, 32].copy_from_slice(initializer.key.as_ref());
            escrow_data_pos += 32;
            msg!("Initializer_account -> OK");

            array_mut_ref![escrow_mut_data, escrow_data_pos, 32].copy_from_slice(taker_account.key.as_ref());
            escrow_data_pos += 32;

            msg!("taker_account -> OK");
        }

            for i in 0..amount_x {
                let i = i as usize;
                {
                    let escrow_mut_data = &mut escrow_account.try_borrow_mut_data()?;

                    initializer_token_account = next_account_info(account_info_iter)?;
                    array_mut_ref![escrow_mut_data, escrow_data_pos, 32].copy_from_slice(initializer_token_account.key.as_ref());
                    escrow_data_pos += 32;
                    msg!("initializer_token_account_x{} : {}", i, initializer_token_account.key);

                    taker_token_account = next_account_info(account_info_iter)?;
                    array_mut_ref![escrow_mut_data, escrow_data_pos, 32].copy_from_slice(taker_token_account.key.as_ref());
                    escrow_data_pos += 32;
                    msg!("taker_token_account_x{} : {}", i, taker_token_account.key);

                    temp_token_account = next_account_info(account_info_iter)?;
                    array_mut_ref![escrow_mut_data, escrow_data_pos, 32].copy_from_slice(temp_token_account.key.as_ref());
                    escrow_data_pos += 32;
                    msg!("temp_token_account_x{} : {}", i, temp_token_account.key);

                    escrow_mut_data[escrow_data_pos] = (lamports_x[i] >> 56) as u8;
                    escrow_data_pos += 1;
                    escrow_mut_data[escrow_data_pos] = (lamports_x[i] >> 48) as u8;
                    escrow_data_pos += 1;
                    escrow_mut_data[escrow_data_pos] = (lamports_x[i] >> 40) as u8;
                    escrow_data_pos += 1;
                    escrow_mut_data[escrow_data_pos] = (lamports_x[i] >> 32) as u8;
                    escrow_data_pos += 1;
                    escrow_mut_data[escrow_data_pos] = (lamports_x[i] >> 24) as u8;
                    escrow_data_pos += 1;
                    escrow_mut_data[escrow_data_pos] = (lamports_x[i] >> 16) as u8;
                    escrow_data_pos += 1;
                    escrow_mut_data[escrow_data_pos] = (lamports_x[i] >> 8) as u8;
                    escrow_data_pos += 1;
                    escrow_mut_data[escrow_data_pos] = lamports_x[i] as u8;
                    escrow_data_pos += 1;
                    msg!("lamports_x{} : {}", i, lamports_x[i]);
                }
    
                transfer_initializer_to_temp_ix = spl_token::instruction::transfer(
                    token_program.key,
                    initializer_token_account.key,
                    temp_token_account.key,
                    initializer.key,
                    &[&initializer.key],
                    lamports_x[i],
                )?;
                msg!("Calling the token program to transfer initializer ---> temp token account");
                invoke(
                    &transfer_initializer_to_temp_ix,
                    &[
                        initializer.clone(),
                        initializer_token_account.clone(),
                        temp_token_account.clone(),
                        token_program.clone(),
                    ],
                )?;

                owner_change_ix = spl_token::instruction::set_authority(
                    token_program.key,
                    temp_token_account.key,
                    Some(&pda),
                    spl_token::instruction::AuthorityType::AccountOwner,
                    escrow_account.key,
                    &[&escrow_account.key],
                )?;
        
                msg!("Calling the token program to transfer token account ownership...");
                invoke(
                    &owner_change_ix,
                    &[
                        token_program.clone(),
                        temp_token_account.clone(),
                        escrow_account.clone(),
                    ],
                )?;

            }

            {
                let escrow_mut_data = &mut escrow_account.try_borrow_mut_data()?;
                for j in 0..amount_y {
                    let j = j as usize;

                    initializer_token_account = next_account_info(account_info_iter)?;
                    array_mut_ref![escrow_mut_data, escrow_data_pos, 32].copy_from_slice(initializer_token_account.key.as_ref());
                    escrow_data_pos += 32;
                    msg!("initializer_token_account_y{} : {}", j, initializer_token_account.key);

                    taker_token_account = next_account_info(account_info_iter)?;
                    array_mut_ref![escrow_mut_data, escrow_data_pos, 32].copy_from_slice(taker_token_account.key.as_ref());
                    escrow_data_pos += 32;
                    msg!("taker_token_account{} : {}", j, taker_token_account.key);

                    escrow_mut_data[escrow_data_pos] = (lamports_y[j] >> 56) as u8;
                    escrow_data_pos += 1;
                    escrow_mut_data[escrow_data_pos] = (lamports_y[j] >> 48) as u8;
                    escrow_data_pos += 1;
                    escrow_mut_data[escrow_data_pos] = (lamports_y[j] >> 40) as u8;
                    escrow_data_pos += 1;
                    escrow_mut_data[escrow_data_pos] = (lamports_y[j] >> 32) as u8;
                    escrow_data_pos += 1;
                    escrow_mut_data[escrow_data_pos] = (lamports_y[j] >> 24) as u8;
                    escrow_data_pos += 1;
                    escrow_mut_data[escrow_data_pos] = (lamports_y[j] >> 16) as u8;
                    escrow_data_pos += 1;
                    escrow_mut_data[escrow_data_pos] = (lamports_y[j] >> 8) as u8;
                    escrow_data_pos += 1;
                    escrow_mut_data[escrow_data_pos] = lamports_y[j] as u8;
                    escrow_data_pos += 1;
                    msg!("lamports_y{} : {}", j, lamports_y[j]);
                }
                msg!("Escrow Mut Data -> {:?}", escrow_mut_data);
            }
        // }

        let system_program_account = next_account_info(account_info_iter)?;
        if (sol_dir == 1) && (lamports) > 0 {
            let sol_ix = system_instruction::transfer(
                initializer.key,
                escrow_account.key,
                lamports,
            );
            invoke(
                &sol_ix,
                &[
                    initializer.clone(),
                    escrow_account.clone(),
                    system_program_account.clone(),
                ],
            )?;
        }

        Ok(())
    }
    //==========================================================================
    fn process_cancel_escrow(
        accounts: &[AccountInfo],
        lamports: u64,
        sol_dir: u8,
        amount_x: u8,
        amount_y: u8,
        lamports_x: [u64; 9],
        lamports_y: [u64; 9],
        program_id: &Pubkey,
    ) -> ProgramResult {

        let account_info_iter = &mut accounts.iter();

        let initializer = next_account_info(account_info_iter)?;
        if !initializer.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        msg!("initializer Pubkey : {}", initializer.key);

        let taker_account = next_account_info(account_info_iter)?;
        msg!("Taker Pubkey : {}", taker_account.key);
        
        let escrow_account = next_account_info(account_info_iter)?;
        msg!("Escrow account Pubkey : {}", escrow_account.key );

        let rent = &Rent::from_account_info(next_account_info(account_info_iter)?)?;

        if !rent.is_exempt(escrow_account.lamports(), escrow_account.data_len()) {
            msg!("Rent error --------> ???");
            return Err(EscrowError::NotRentExempt.into());
        }
        msg!("Rent OK -------------->");

        let token_program = next_account_info(account_info_iter)?;
        msg!("token_program : {}", token_program.key);
        let pda_account = next_account_info(account_info_iter)?;
        let (pda, _nonce) = Pubkey::find_program_address(&[b"escrow"], program_id);

        {
            let escrow_data = &escrow_account.try_borrow_data()?;
            let mut escrow_data_pos = 0;

            if escrow_data[escrow_data_pos] == 0 {
                msg!("escrow_account data does not exist !!!");
                return Err(ProgramError::AccountAlreadyInitialized);
            }
            escrow_data_pos += 1;

            if escrow_data[escrow_data_pos] != amount_x as u8 {
                msg!("amount_x is not the same !");
                return Err(EscrowError::InvalidAmount.into());
            }
            escrow_data_pos += 1;

            if escrow_data[escrow_data_pos] != amount_y as u8 {
                msg!("amount_y is not the same !");
                return Err(EscrowError::InvalidAmount.into());
            }
            escrow_data_pos += 1;

            if escrow_data[escrow_data_pos] != sol_dir as u8 {
                msg!("sol_dir is not the same !");
                return Err(EscrowError::InvalidAmount.into());
            }
            escrow_data_pos += 1;

            let mut temp_lamports = escrow_data[escrow_data_pos] as u64;
            escrow_data_pos += 1;
            temp_lamports = (temp_lamports << 8) + escrow_data[escrow_data_pos] as u64;
            escrow_data_pos += 1;
            temp_lamports = (temp_lamports << 8) + escrow_data[escrow_data_pos] as u64;
            escrow_data_pos += 1;
            temp_lamports = (temp_lamports << 8) + escrow_data[escrow_data_pos] as u64;
            escrow_data_pos += 1;
            temp_lamports = (temp_lamports << 8) + escrow_data[escrow_data_pos] as u64;
            escrow_data_pos += 1;
            temp_lamports = (temp_lamports << 8) + escrow_data[escrow_data_pos] as u64;
            escrow_data_pos += 1;
            temp_lamports = (temp_lamports << 8) + escrow_data[escrow_data_pos] as u64;
            escrow_data_pos += 1;
            temp_lamports = (temp_lamports << 8) + escrow_data[escrow_data_pos] as u64;
            escrow_data_pos += 1;

            msg!("Lamports --> {}, temp_lamports --> {}", lamports, temp_lamports);

            if lamports != temp_lamports {
                msg!("lamports is not the same !");
                return Err(EscrowError::InvalidAccount.into());
            }

            msg!("Lamports OK -------------->");

            let mut initializer_token_account;
            let mut taker_token_account;
            let mut temp_token_account;

            let mut transfer_temp_to_initializer_ix;
            let mut close_escrow_temp_acc_ix;

            if initializer.key.as_ref() != array_ref!(escrow_data, escrow_data_pos, 32) {
                msg!("initializer pubkey is not the same !");
                return Err(EscrowError::InvalidAccount.into());
            }
            escrow_data_pos += 32;
            msg!("Initializer Account OK -------------->");

            if taker_account.key.as_ref() != array_ref!(escrow_data, escrow_data_pos, 32) {
                msg!("taker pubkey is not the same !");
                return Err(EscrowError::InvalidAccount.into());
            }
            escrow_data_pos += 32;
            msg!("Taker Account OK -------------->");

            for i in 0..amount_x {
                let i = i as usize;

                initializer_token_account = next_account_info(account_info_iter)?;
                if initializer_token_account.key.as_ref() != array_ref!(escrow_data, escrow_data_pos, 32) {
                    msg!("initializer x token account pubkey{} is not the same !", i);
                    return Err(EscrowError::InvalidAccount.into());
                }
                escrow_data_pos += 32;
                msg!("initializer x token account pubkey{} is okay !", i);

                taker_token_account = next_account_info(account_info_iter)?;
                if taker_token_account.key.as_ref() != array_ref!(escrow_data, escrow_data_pos, 32) {
                    msg!("taker x token account pubkey{} is not the same !", i);
                    return Err(EscrowError::InvalidAccount.into());
                }
                escrow_data_pos += 32;
                msg!("taker x token account pubkey{} is okay !", i);

                temp_token_account = next_account_info(account_info_iter)?;
                if temp_token_account.key.as_ref() != array_ref!(escrow_data, escrow_data_pos, 32) {
                    msg!("temp x token account pubkey{} is not the same !", i);
                    return Err(EscrowError::InvalidAccount.into());
                }
                escrow_data_pos += 32;
                msg!("temp x token account pubkey{} is okay !", i);
                 
                temp_lamports = escrow_data[escrow_data_pos] as u64;
                escrow_data_pos += 1;
                temp_lamports = (temp_lamports << 8) + escrow_data[escrow_data_pos] as u64;
                escrow_data_pos += 1;
                temp_lamports = (temp_lamports << 8) + escrow_data[escrow_data_pos] as u64;
                escrow_data_pos += 1;
                temp_lamports = (temp_lamports << 8) + escrow_data[escrow_data_pos] as u64;
                escrow_data_pos += 1;
                temp_lamports = (temp_lamports << 8) + escrow_data[escrow_data_pos] as u64;
                escrow_data_pos += 1;
                temp_lamports = (temp_lamports << 8) + escrow_data[escrow_data_pos] as u64;
                escrow_data_pos += 1;
                temp_lamports = (temp_lamports << 8) + escrow_data[escrow_data_pos] as u64;
                escrow_data_pos += 1;
                temp_lamports = (temp_lamports << 8) + escrow_data[escrow_data_pos] as u64;
                escrow_data_pos += 1;
                msg!("Lamports_x --> {}, temp_lamport --> {}", lamports_x[i], temp_lamports);

                if lamports_x[i] != temp_lamports {
                    msg!("lamports_x is not the same !");
                    return Err(EscrowError::InvalidAccount.into());
                }
                msg!("Lamports_x{} OK -------------->", i);
    
                transfer_temp_to_initializer_ix = spl_token::instruction::transfer(
                    token_program.key,
                    temp_token_account.key,
                    initializer_token_account.key,
                    &pda,
                    &[&pda],
                    lamports_x[i],
                )?;
                msg!("Calling the token program to transfer tokens to the Initializer token account...");
                invoke_signed(
                    &transfer_temp_to_initializer_ix,
                    &[
                        pda_account.clone(),
                        token_program.clone(),
                        temp_token_account.clone(),
                        initializer_token_account.clone(),
                    ],
                    &[&[&b"escrow"[..], &[_nonce]]],
                )?;

                close_escrow_temp_acc_ix = spl_token::instruction::close_account(
                    token_program.key,
                    temp_token_account.key,
                    initializer_token_account.key,
                    &pda,
                    &[&pda],
                )?;
                msg!("Calling the token program to close pda's temp account...");
                invoke_signed(
                    &close_escrow_temp_acc_ix,
                    &[
                        pda_account.clone(),
                        token_program.clone(),
                        temp_token_account.clone(),
                        initializer_token_account.clone(),
                    ],
                    &[&[&b"escrow"[..], &[_nonce]]],
                )?;

            }

            //---- check taker account & lamports

            // let escrow_mut_data = &mut escrow_account.try_borrow_mut_data()?;
            for j in 0..amount_y {
                let j = j as usize;

                initializer_token_account = next_account_info(account_info_iter)?;
                if initializer_token_account.key.as_ref() != array_ref!(escrow_data, escrow_data_pos, 32) {
                    msg!("initializer y token account pubkey{} is not the same !", j);
                    return Err(EscrowError::InvalidAccount.into());
                }
                escrow_data_pos += 32;
                msg!("initializer y token account pubkey{} is okay !", j);

                taker_token_account = next_account_info(account_info_iter)?;
                if taker_token_account.key.as_ref() != array_ref!(escrow_data, escrow_data_pos, 32) {
                    msg!("taker y token account pubkey{} is not the same !", j);
                    return Err(EscrowError::InvalidAccount.into());
                }
                escrow_data_pos += 32;
                msg!("taker y token account pubkey{} is okay !", j);

                temp_lamports = escrow_data[escrow_data_pos] as u64;
                escrow_data_pos += 1;
                temp_lamports = (temp_lamports << 8) + escrow_data[escrow_data_pos] as u64;
                escrow_data_pos += 1;
                temp_lamports = (temp_lamports << 8) + escrow_data[escrow_data_pos] as u64;
                escrow_data_pos += 1;
                temp_lamports = (temp_lamports << 8) + escrow_data[escrow_data_pos] as u64;
                escrow_data_pos += 1;
                temp_lamports = (temp_lamports << 8) + escrow_data[escrow_data_pos] as u64;
                escrow_data_pos += 1;
                temp_lamports = (temp_lamports << 8) + escrow_data[escrow_data_pos] as u64;
                escrow_data_pos += 1;
                temp_lamports = (temp_lamports << 8) + escrow_data[escrow_data_pos] as u64;
                escrow_data_pos += 1;
                temp_lamports = (temp_lamports << 8) + escrow_data[escrow_data_pos] as u64;
                escrow_data_pos += 1;
                msg!("Lamports_y --> {}, temp_lamports --> {}", lamports_y[j], temp_lamports);

                if lamports_y[j] != temp_lamports {
                    msg!("lamports_y is not the same !");
                    return Err(EscrowError::InvalidAccount.into());
                }
                msg!("Lamports_y{} OK -------------->", j);
            }
            // msg!("Escrow Mut Data -> {:?}", escrow_mut_data);
        }

        msg!("Closing the escrow account...");
        **initializer.try_borrow_mut_lamports()? = initializer
            .lamports()
            .checked_add(escrow_account.lamports())
            .ok_or(EscrowError::AmountOverflow)?;
        **escrow_account.try_borrow_mut_lamports()? = 0;
        *escrow_account.try_borrow_mut_data()? = &mut [];

        Ok(())
    }

    //==========================================================================
    fn process_exchange(
        accounts: &[AccountInfo],
        lamports: u64,
        sol_dir: u8,
        amount_x: u8,
        amount_y: u8,
        lamports_x: [u64; 9],
        lamports_y: [u64; 9],
        program_id: &Pubkey,
    ) -> ProgramResult {

        let account_info_iter = &mut accounts.iter();

        let initializer = next_account_info(account_info_iter)?;
        msg!("initializer Pubkey : {}", initializer.key);

        let taker_account = next_account_info(account_info_iter)?;
        if !taker_account.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }
        msg!("Taker Pubkey : {}", taker_account.key);
        
        let escrow_account = next_account_info(account_info_iter)?;
        msg!("Escrow account Pubkey : {}", escrow_account.key );

        let rent = &Rent::from_account_info(next_account_info(account_info_iter)?)?;

        if !rent.is_exempt(escrow_account.lamports(), escrow_account.data_len()) {
            msg!("Rent error --------> ???");
            return Err(EscrowError::NotRentExempt.into());
        }
        msg!("Rent OK -------------->");

        let token_program = next_account_info(account_info_iter)?;
        msg!("token_program : {}", token_program.key);
        let pda_account = next_account_info(account_info_iter)?;
        let (pda, _nonce) = Pubkey::find_program_address(&[b"escrow"], program_id);

        {
            let escrow_data = &escrow_account.try_borrow_data()?;
            let mut escrow_data_pos = 0;

            if escrow_data[escrow_data_pos] == 0 {
                msg!("escrow_account data does not exist !!!");
                return Err(ProgramError::AccountAlreadyInitialized);
            }
            escrow_data_pos += 1;

            if escrow_data[escrow_data_pos] != amount_x as u8 {
                msg!("amount_x is not the same !");
                return Err(EscrowError::InvalidAmount.into());
            }
            escrow_data_pos += 1;

            if escrow_data[escrow_data_pos] != amount_y as u8 {
                msg!("amount_y is not the same !");
                return Err(EscrowError::InvalidAmount.into());
            }
            escrow_data_pos += 1;

            if escrow_data[escrow_data_pos] != sol_dir as u8 {
                msg!("sol_dir is not the same !");
                return Err(EscrowError::InvalidAmount.into());
            }
            escrow_data_pos += 1;

            let mut temp_lamports = escrow_data[escrow_data_pos] as u64;
            escrow_data_pos += 1;
            temp_lamports = (temp_lamports << 8) + escrow_data[escrow_data_pos] as u64;
            escrow_data_pos += 1;
            temp_lamports = (temp_lamports << 8) + escrow_data[escrow_data_pos] as u64;
            escrow_data_pos += 1;
            temp_lamports = (temp_lamports << 8) + escrow_data[escrow_data_pos] as u64;
            escrow_data_pos += 1;
            temp_lamports = (temp_lamports << 8) + escrow_data[escrow_data_pos] as u64;
            escrow_data_pos += 1;
            temp_lamports = (temp_lamports << 8) + escrow_data[escrow_data_pos] as u64;
            escrow_data_pos += 1;
            temp_lamports = (temp_lamports << 8) + escrow_data[escrow_data_pos] as u64;
            escrow_data_pos += 1;
            temp_lamports = (temp_lamports << 8) + escrow_data[escrow_data_pos] as u64;
            escrow_data_pos += 1;

            msg!("Lamports --> {}, temp_lamports --> {}", lamports, temp_lamports);

            if lamports != temp_lamports {
                msg!("lamports is not the same !");
                return Err(EscrowError::InvalidAccount.into());
            }

            msg!("Lamports OK -------------->");

            let mut initializer_token_account;
            let mut taker_token_account;
            let mut temp_token_account;

            let mut transfer_taker_to_initializer_ix;
            let mut transfer_temp_to_taker_ix;
            let mut close_escrow_temp_acc_ix;

            if initializer.key.as_ref() != array_ref!(escrow_data, escrow_data_pos, 32) {
                msg!("initializer pubkey is not the same !");
                return Err(EscrowError::InvalidAccount.into());
            }
            escrow_data_pos += 32;
            msg!("Initializer Account OK -------------->");

            if taker_account.key.as_ref() != array_ref!(escrow_data, escrow_data_pos, 32) {
                msg!("taker pubkey is not the same ! : {:?}", array_ref!(escrow_data, escrow_data_pos, 32));
                return Err(EscrowError::InvalidAccount.into());
            }
            escrow_data_pos += 32;
            msg!("Taker Account OK -------------->");

            for i in 0..amount_x {
                let i = i as usize;

                initializer_token_account = next_account_info(account_info_iter)?;
                if initializer_token_account.key.as_ref() != array_ref!(escrow_data, escrow_data_pos, 32) {
                    msg!("initializer x token account pubkey{} is not the same !", i);
                    return Err(EscrowError::InvalidAccount.into());
                }
                escrow_data_pos += 32;
                msg!("initializer x token account pubkey{} is okay !", i);

                taker_token_account = next_account_info(account_info_iter)?;
                if taker_token_account.key.as_ref() != array_ref!(escrow_data, escrow_data_pos, 32) {
                    msg!("taker x token account pubkey{} is not the same !", i);
                    return Err(EscrowError::InvalidAccount.into());
                }
                escrow_data_pos += 32;
                msg!("taker x token account pubkey{} is okay !", i);

                temp_token_account = next_account_info(account_info_iter)?;
                if temp_token_account.key.as_ref() != array_ref!(escrow_data, escrow_data_pos, 32) {
                    msg!("temp x token account pubkey{} is not the same !", i);
                    return Err(EscrowError::InvalidAccount.into());
                }
                escrow_data_pos += 32;
                msg!("temp x token account pubkey{} is okay !", i);

                temp_lamports = escrow_data[escrow_data_pos] as u64;
                escrow_data_pos += 1;
                temp_lamports = (temp_lamports << 8) + escrow_data[escrow_data_pos] as u64;
                escrow_data_pos += 1;
                temp_lamports = (temp_lamports << 8) + escrow_data[escrow_data_pos] as u64;
                escrow_data_pos += 1;
                temp_lamports = (temp_lamports << 8) + escrow_data[escrow_data_pos] as u64;
                escrow_data_pos += 1;
                temp_lamports = (temp_lamports << 8) + escrow_data[escrow_data_pos] as u64;
                escrow_data_pos += 1;
                temp_lamports = (temp_lamports << 8) + escrow_data[escrow_data_pos] as u64;
                escrow_data_pos += 1;
                temp_lamports = (temp_lamports << 8) + escrow_data[escrow_data_pos] as u64;
                escrow_data_pos += 1;
                temp_lamports = (temp_lamports << 8) + escrow_data[escrow_data_pos] as u64;
                escrow_data_pos += 1;
                msg!("Lamports_x --> {}, temp_lamport --> {}", lamports_x[i], temp_lamports);

                if lamports_x[i] != temp_lamports {
                    msg!("lamports_x is not the same !");
                    return Err(EscrowError::InvalidAccount.into());
                }
                msg!("Lamports_x{} OK -------------->", i);

                transfer_temp_to_taker_ix = spl_token::instruction::transfer(
                    token_program.key,
                    temp_token_account.key,
                    taker_token_account.key,
                    &pda,
                    &[&pda],
                    lamports_x[i],
                )?;
                msg!("Calling the token program to exchange tokens ...");
                invoke_signed(
                    &transfer_temp_to_taker_ix,
                    &[
                        pda_account.clone(),
                        token_program.clone(),
                        temp_token_account.clone(),
                        taker_token_account.clone(),
                    ],
                    &[&[&b"escrow"[..], &[_nonce]]],
                )?;

                close_escrow_temp_acc_ix = spl_token::instruction::close_account(
                    token_program.key,
                    temp_token_account.key,
                    initializer_token_account.key,
                    &pda,
                    &[&pda],
                )?;
                msg!("Calling the token program to close pda's temp account...");
                invoke_signed(
                    &close_escrow_temp_acc_ix,
                    &[
                        pda_account.clone(),
                        token_program.clone(),
                        temp_token_account.clone(),
                        initializer_token_account.clone(),
                    ],
                    &[&[&b"escrow"[..], &[_nonce]]],
                )?;

            }

            for j in 0..amount_y {
                let j = j as usize;

                initializer_token_account = next_account_info(account_info_iter)?;
                if initializer_token_account.key.as_ref() != array_ref!(escrow_data, escrow_data_pos, 32) {
                    msg!("initializer y token account pubkey{} is not the same !", j);
                    return Err(EscrowError::InvalidAccount.into());
                }
                escrow_data_pos += 32;
                msg!("initializer y token account pubkey{} is okay !", j);

                taker_token_account = next_account_info(account_info_iter)?;
                if taker_token_account.key.as_ref() != array_ref!(escrow_data, escrow_data_pos, 32) {
                    msg!("taker y token account pubkey{} is not the same !", j);
                    return Err(EscrowError::InvalidAccount.into());
                }
                escrow_data_pos += 32;
                msg!("taker y token account pubkey{} is okay !", j);

                temp_lamports = escrow_data[escrow_data_pos] as u64;
                escrow_data_pos += 1;
                temp_lamports = (temp_lamports << 8) + escrow_data[escrow_data_pos] as u64;
                escrow_data_pos += 1;
                temp_lamports = (temp_lamports << 8) + escrow_data[escrow_data_pos] as u64;
                escrow_data_pos += 1;
                temp_lamports = (temp_lamports << 8) + escrow_data[escrow_data_pos] as u64;
                escrow_data_pos += 1;
                temp_lamports = (temp_lamports << 8) + escrow_data[escrow_data_pos] as u64;
                escrow_data_pos += 1;
                temp_lamports = (temp_lamports << 8) + escrow_data[escrow_data_pos] as u64;
                escrow_data_pos += 1;
                temp_lamports = (temp_lamports << 8) + escrow_data[escrow_data_pos] as u64;
                escrow_data_pos += 1;
                temp_lamports = (temp_lamports << 8) + escrow_data[escrow_data_pos] as u64;
                escrow_data_pos += 1;
                msg!("Lamports_y --> {}, temp_lamports --> {}", lamports_y[j], temp_lamports);

                if lamports_y[j] != temp_lamports {
                    msg!("lamports_y is not the same !");
                    return Err(EscrowError::InvalidAccount.into());
                }
                msg!("Lamports_y{} OK -------------->", j);
                
                transfer_taker_to_initializer_ix = spl_token::instruction::transfer(
                    token_program.key,
                    taker_token_account.key,
                    initializer_token_account.key,
                    taker_account.key,
                    &[&taker_account.key],
                    lamports_y[j]
                )?;
                msg!("Calling the token program to transfer tokens to the Initializer token account...");
                invoke(
                    &transfer_taker_to_initializer_ix,
                    &[
                        taker_account.clone(),
                        taker_token_account.clone(),
                        initializer_token_account.clone(),
                        token_program.clone(),
                    ],
                )?;

            }

        }

        let system_program_account = next_account_info(account_info_iter)?;
        
        if (sol_dir == 1) && (lamports) > 0 {
            **escrow_account.try_borrow_mut_lamports()? -= lamports;
            **taker_account.try_borrow_mut_lamports()? += lamports;
        }

        if (sol_dir == 2) && (lamports) > 0 {
            let sol_ix = system_instruction::transfer(
                taker_account.key,
                escrow_account.key,
                lamports,
            );
            invoke(
                &sol_ix,
                &[
                    taker_account.clone(),
                    escrow_account.clone(),
                    system_program_account.clone(),
                ],
            )?;
        }

        msg!("Closing the escrow account...");
        **initializer.try_borrow_mut_lamports()? = initializer
            .lamports()
            .checked_add(escrow_account.lamports())
            .ok_or(EscrowError::AmountOverflow)?;
        **escrow_account.try_borrow_mut_lamports()? = 0;
        *escrow_account.try_borrow_mut_data()? = &mut [];

        Ok(())
    }
}