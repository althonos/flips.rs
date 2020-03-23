#![cfg_attr(not(feature = "std"), no_std)]
#![allow(bad_style)]

extern crate libc;

#[cfg(feature = "crc32fast")]
extern crate crc32fast;

#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
extern crate quickcheck_macros;

pub mod bps;
pub mod ips;
pub mod ups;

mod crc32;
#[cfg(test)]
mod test_utils;

/// The representation of a memory slice in the Flips API.
///
/// Equivalent to a [`slice`](https://doc.rust-lang.org/std/primitive.slice.html),
/// but without compile-time checks for mutability and ownership.
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct mem {
    pub ptr: *mut u8,
    pub len: libc::size_t,
}

impl mem {
    pub fn new(ptr: *mut u8, len: libc::size_t) -> Self {
        Self { ptr, len }
    }
}

impl Default for mem {
    fn default() -> Self {
        Self::new(core::ptr::null_mut(), 0)
    }
}

impl AsRef<[u8]> for mem {
    fn as_ref(&self) -> &[u8] {
        unsafe {
            core::slice::from_raw_parts(self.ptr as *const _, self.len)
        }
    }
}
