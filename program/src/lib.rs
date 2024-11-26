#![feature(panic_info_message)]

use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    log::sol_log,
    msg,
    pubkey::Pubkey,
    program_error::ProgramError,
};

entrypoint!(process_instruction);

#[inline(never)]
fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {

    let a = instruction_data[20545];

    Ok(())
}

#[no_mangle]
fn custom_panic(info: &core::panic::PanicInfo<'_>) {
    let loc = info.location().unwrap();
    let filename = loc.file();
    let mut msg_line  = vec![32; filename.len() + 30];
    let start_p = "Panicked at :";
    let dst = msg_line.as_mut_ptr();
    let mut num : [u8; 10] = [0; 10];
    unsafe {
        let src = start_p.as_ptr();
        std::ptr::copy_nonoverlapping(
            src,
            dst,
            start_p.len()
        );
    }
    unsafe {
        let dst = dst.add(14);
        let src = filename.as_ptr();
        std::ptr::copy_nonoverlapping(
            src,
            dst,
            filename.len()
        );
    }

    let start = convert(&mut num, loc.line());
    unsafe {
        let dst = dst.add(filename.len()+14);
        *dst = 58;
        let dst = dst.add(1);
        let src = num.as_ptr().add(start);
        std::ptr::copy_nonoverlapping(
            src,
            dst,
            10 - start,
        );
    }

    let start = convert(&mut num, loc.column());
    unsafe {
        let dst = dst.add(filename.len()+25-start);
        *dst = 58;
        let dst = dst.add(1);
        let src = num.as_ptr().add(start);
        std::ptr::copy_nonoverlapping(
            src,
            dst,
            10 - start,
        );
    }

    unsafe {
        solana_program::syscalls::sol_log_(msg_line.as_ptr(), msg_line.len() as u64);
    }

    if let Some(Some(mm)) = info.message().map(|mes| mes.as_str()) {
        unsafe {
            solana_program::syscalls::sol_log_(mm.as_ptr(), mm.len() as u64);
        }   
    }
}

#[inline(never)]
fn convert(x: &mut [u8; 10], mut val: u32) -> usize {
    let mut i = 9;
    while val > 0 {
        x[i] = (val % 10) as u8 + 48;
        val /= 10;
        i -= 1;
    }

    i + 1
}
