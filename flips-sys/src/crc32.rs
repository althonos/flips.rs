#[no_mangle]
pub extern "C" fn crc32(data: *const u8, len: libc::size_t) -> u32 {
    unsafe {
        let mut hasher = crc32fast::Hasher::new();
        hasher.update(core::slice::from_raw_parts(data, len));
        hasher.finalize()
   }
}
