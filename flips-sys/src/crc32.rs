#[no_mangle]
pub extern "C" fn crc32(data: *const u8, len: libc::size_t) -> u32 {
    unsafe {
        let mut hasher = crc32fast::Hasher::new();
        hasher.update(core::slice::from_raw_parts(data, len));
        hasher.finalize()
   }
}

#[no_mangle]
pub extern "C" fn crc32_update(data: *const u8, len: libc::size_t, crc: u32) -> u32 {
   unsafe {
       let mut hasher = crc32fast::Hasher::new_with_initial(crc);
       hasher.update(core::slice::from_raw_parts(data, len));
       hasher.finalize()
   }
}
