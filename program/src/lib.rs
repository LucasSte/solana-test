
extern "C" {
   pub fn sol_memcpy_(dest: *mut u8, src: *const u8, n: u64);
   pub fn sol_memmove_(dest: *mut u8, src: *const u8, n: u64);
   pub fn sol_memset_(s: *mut u8, c: u8, n: u64);
   pub fn sol_memcmp_(s1: *const u8, s2: *const u8, n: u64, result: *mut i32);
}



#[no_mangle]
pub unsafe extern "C" fn entrypoint(input: *mut u8) -> u64 {
   let mut a: [u8; 32] = [1; 32];
   let mut b: [u8; 32] = [1; 32];
   b[31] = 5;
   
   std::hint::black_box(&mut a);
   std::hint::black_box(&mut b);
   let mut sz: usize = 5;
   std::hint::black_box(&mut sz);

   // if a.get_unchecked(..sz) == b.get_unchecked(..sz) {
   //    return 0;
   // } else {
   //    return 1;
   // }
   // 97 cus
   // 47 CUs
      
   let mut result: i32 = 0;

   sol_memcmp_(a.as_ptr(), b.as_ptr(), sz as u64, &mut result);

   result as u64
}


// #[no_mangle]
// pub unsafe extern "C" fn entrypoint(input: *mut u8) -> u64 {
//    let n1 = u64::from_le_bytes(std::slice::from_raw_parts(input, 8).try_into().unwrap());
//    let n2 = u64::from_le_bytes(std::slice::from_raw_parts(input.add(16), 8).try_into().unwrap());
//    let src = input.add(32);
//    let dst = input.add(n1 as usize);

//    // memcpy
//    // 7 instr to check + 10 to syscall + 5 after syscall = 22
//    // 7 instr to check + 22 = 29
//    //
//    // Best case 7 + 5 = 12
//    // Intermediate 7 + 5 + 9 = 21
//    // Worst case 29
//    //
//    // We need at least 11 loads/stores before calling = 22 instr
//    std::ptr::copy_nonoverlapping(src, dst, n2 as usize);

//    let src = input.add(64);
//    let dst = input.add(90);

//    // memmove
//    // 7 instr to check + 10 to syscall + 5 after syscall = 22
//    // Case 1: 8 + 22 = 30
//    // Case 2: 8 + 33 = 41
//    //
//    // We need at least 11 loads/stores before calling = 22 instr
//    std::ptr::copy(src, dst, n2 as usize);

//    let mut p = std::slice::from_raw_parts_mut(input.add(128), n1 as usize);
//    // memset
//    // 7 instr to check + 10 to syscall + 5 after syscall = 22
//    // Best case: 19
//    // Worst case: 22
//    p.fill(0);
   
//    let p1 = std::slice::from_raw_parts(input.add(256), n2 as usize);
//    let p2 = std::slice::from_raw_parts(input.add(512), n2 as usize);

//    // memcmp
//    // 7 instr to check + 10 to syscall + 5 after syscall = 22
//    // Best case: 17
//    // Worst case: 28
//    if p1 == p2 {
//     1
//    } else {
//     0
//    }
// }

#[no_mangle]
fn custom_panic(info: &core::panic::PanicInfo<'_>) {

}

