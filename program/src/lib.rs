

#[no_mangle]
pub unsafe extern "C" fn entrypoint(input: *mut u8) -> u64 {
    let sparkle_heart = [240, 159, 146, 150];
    let result_str = std::str::from_utf8(&sparkle_heart).unwrap();
    assert_eq!(4, result_str.len());
    return 0;
}
