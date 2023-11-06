static mut STRING_ARENA: [i8; 1024 * 32] = [0; 1024 * 32];
static mut STRING_LEN: usize = 0;

/// # Safety
pub unsafe fn ptr() -> *mut i8 {
    STRING_ARENA.as_mut_ptr()
}

/// # Safety
pub unsafe fn len() -> *mut usize {
    &mut STRING_LEN
}

/// # Safety
pub unsafe fn reset_len() -> *mut usize {
    STRING_LEN = STRING_ARENA.len();
    &mut STRING_LEN
}

/// # Safety
pub unsafe fn read_string() -> String {
    let buf = std::slice::from_raw_parts(ptr().cast::<u8>(), *len());

    String::from_utf8_lossy(buf).into_owned()
}
