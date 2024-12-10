#![feature(panic_info_message)]

use pinocchio_log::logger::Logger;
use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
};


entrypoint!(process_instruction);

fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {

    // let a = instruction_data[85034];
    // let a : Option<u64> = None;
    // let b = a.unwrap();
    Ok(())
}

#[no_mangle]
fn custom_panic(info: &core::panic::PanicInfo<'_>) {
    if let Some(loc) = info.location() {
        const MAX_LEN : usize = 200;
        let filename = loc.file();
        // MAX_LEN for filename + 3 (ellipsis string) + 11 (line number + separator)
        // + 11 (column number + separator) + 13 (panic string) + 15 extra bytes
        let mut logger = Logger::<{MAX_LEN + 53}>::default();

        logger.append("Panicket at: "); // 13 bytes
        
        let trunc_name = if filename.as_bytes().len() > MAX_LEN {
            logger.append("...");
            &filename[(filename.as_bytes().len() - MAX_LEN)..]
        } else {
            &filename[..]
        };

        logger.append(trunc_name);

        logger.append(":");
        logger.append(loc.line());
        logger.append(":");
        logger.append(loc.column());
        
        logger.log();
    }

    if let Some(Some(mm)) = info.message().map(|mes| mes.as_str()) {
        let mes = mm.as_bytes();
        // SAFETY: We assumed the panic message is well formed.
        unsafe {
            solana_program::syscalls::sol_log_(mes.as_ptr(), mes.len() as u64);
        }
    }
}


