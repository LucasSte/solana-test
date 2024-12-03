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

    Ok(())
}

// 11k => 11536 bytes
// 13k => 12912 bytes
// No vec:
// 12k  => 12616 bytes

#[no_mangle]
fn custom_panic(info: &core::panic::PanicInfo<'_>) {
    if let Some(loc) = info.location() {
        let filename = loc.file().as_bytes();
        let mut msg_line = vec![32; filename.len() + 25];
    
        let panic_str = "Panicked at: ".as_bytes();
        let dst = msg_line.as_mut_ptr();
        unsafe {
            let src = panic_str.as_ptr();
            std::ptr::copy_nonoverlapping(
                src,
                dst,
                panic_str.len(),
            );

            let dst = dst.add(panic_str.len());
            let src = filename.as_ptr();
            std::ptr::copy_nonoverlapping(
                src,
                dst,
                filename.len(),
            );

            let dst = dst.add(filename.len());
            let written_bytes = write_num(loc.line(), dst);
            let dst = dst.add(written_bytes);
            let _ = write_num(loc.column(), dst);

            solana_program::syscalls::sol_log_(msg_line.as_ptr(), msg_line.len() as u64);

            if let Some(Some(mm)) = info.message().map(|mes| mes.as_str()) {
                let mes = mm.as_bytes();
                solana_program::syscalls::sol_log_(mes.as_ptr(), mes.len() as u64);
            }
        }
    } 
}

#[inline(never)]
unsafe fn write_num(mut num: u32, dst: *mut u8) -> usize{
    let mut buf : [u8; 10] = [0; 10];
    let mut buf_ptr = buf.as_mut_ptr().add(11);
    while num > 0 {
        buf_ptr = buf_ptr.sub(1);
        *buf_ptr = (num % 10) as u8 + 48;
        num /= 10;
    }

    *dst = 58;
    let dst = dst.add(1);
    let len = buf_ptr.offset_from(buf.as_ptr()) as usize;
    std::ptr::copy_nonoverlapping(
        buf_ptr,
        dst,
        len,
    );
    len + 1
}
