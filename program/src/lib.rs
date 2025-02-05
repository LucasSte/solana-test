
extern "C" {
   pub fn sol_memcpy_(dest: *mut u8, src: *const u8, n: u64);
   pub fn sol_memmove_(dest: *mut u8, src: *const u8, n: u64);
   pub fn sol_memset_(s: *mut u8, c: u8, n: u64);
   pub fn sol_memcmp_(s1: *const u8, s2: *const u8, n: u64, result: *mut i32);
}

// memcmp
// 47 CUs total => 47 - 37 = 10 CUs for syscall
// 96 CUs total => 96 - 36 = 60 CUs for non-syscall

// memset
// 42 CUs total => 42 - 32 = 10 CUs for syscall
// 74 CUs total => 74 - 35 = 39 CUs for non-syscall

// memmove
// 41 CUs total => 41 - 31 = 10 CUs for syscall
// 93 CUs total => 93 - 33 = 60 CUs for non-syscall

// memcpy
// 41 CUs total => 41 - 31 = 10 CUs for syscall
// 84 CUs total => 84 - 31 = 53 CUs for non-syscall

#[no_mangle]
pub unsafe extern "C" fn entrypoint(input: *mut u8) -> u64 {
   let mut a: [u8; 32] = [1; 32];
   let mut b: [u8; 32] = [1; 32];
   b[31] = 5;
   
   std::hint::black_box(&mut a);
   std::hint::black_box(&mut b);
   let mut sz: usize = 5;
   std::hint::black_box(&mut sz);

//    memcmp      
//    let mut result: i32 = 0;
//    sol_memcmp_(a.as_ptr(), b.as_ptr(), sz as u64, &mut result);
//    result as u64
//    ---
//     (a.get_unchecked(..sz) == b.get_unchecked(..sz)) as u64


    // memset
    // sol_memset_(a.as_mut_ptr(), 3, sz as u64);
    // ---
    // a.get_unchecked_mut(..sz).fill(2);
    // *a.get_unchecked(3) as u64
    
    // memmove
//    sol_memmove_(a.as_mut_ptr(), b.as_ptr(), sz as u64);
//    a[3] as u64
   // ---
//    std::ptr::copy(a.as_ptr(), a.as_mut_ptr().add(1), sz as usize);
//    *a.get_unchecked(3) as u64

    // memcpy
    // sol_memcpy_(a.as_mut_ptr(), b.as_ptr(), sz as u64);
    // a[3] as u64
    std::ptr::copy_nonoverlapping(a.as_ptr(), b.as_mut_ptr(), sz as usize);
    *b.get_unchecked(3) as u64
}


#[no_mangle]
fn custom_panic(info: &core::panic::PanicInfo<'_>) {

}

