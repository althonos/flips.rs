#![cfg_attr(feature = "_doc", feature(doc_cfg, external_doc))]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
extern crate err_derive;
extern crate flips_sys;

mod ips;
mod ups;
mod bps;

pub use self::bps::*;
pub use self::ips::*;
pub use self::ups::*;

use core::ops::Deref;

// ---------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "std", derive(err_derive::Error))]
pub enum Error {
    /// Attempted to apply a patch not made for the input.
    #[cfg_attr(feature = "std", error(display = "patch is not made for the input"))]
    NotThis,
    /// Attempted to apply a patch to the output ROM.
    #[cfg_attr(feature = "std", error(display = "attempted to patch the output ROM"))]
    ToOutput,
    /// The patch is invalid or malformed.
    #[cfg_attr(feature = "std", error(display = "patch is invalid or malformed"))]
    Invalid,
    /// The patch is technically valid, but seems scrambled.
    #[cfg_attr(feature = "std", error(display = "patch is valid but seems scrambled or corrupted"))]
    Scrambled,
    /// Attempted to create a patch from identical buffers.
    #[cfg_attr(feature = "std", error(display = "attempted to create a patch from identical buffers"))]
    Identical,
    #[cfg_attr(feature = "std", error(display = "requested a size larger than `libc::size_t`"))]
    TooBig,
    #[cfg_attr(feature = "std", error(display = "memory allocation failed"))]
    OutOfMem,
    #[cfg_attr(feature = "std", error(display = "patch creation was canceled"))]
    Canceled,
}

impl Error {
    /// Attempt to create an `Error` from a raw `ipserror`.
    fn from_ips(e: flips_sys::ips::ipserror) -> Option<Error> {
        use flips_sys::ips::ipserror::*;
        match e {
            ips_ok => None,
            ips_notthis => Some(Error::NotThis),
            ips_thisout => Some(Error::ToOutput),
            ips_scrambled => Some(Error::Scrambled),
            ips_invalid => Some(Error::Invalid),
            ips_16MB => Some(Error::OutOfMem), // FIXME?
            ips_identical => Some(Error::Identical),
            ips_shut_up_gcc => unreachable!("{:?} should never be used !", e),
        }
    }

    /// Attempt to create an `Error` from a raw `upserror`.
    fn from_ups(e: flips_sys::ups::upserror) -> Option<Error> {
        use flips_sys::ups::upserror::*;
        match e {
            ups_ok => None,
            ups_not_this => Some(Error::NotThis),
            ups_broken => Some(Error::Invalid),
            ups_identical => Some(Error::Identical),
            ups_too_big => Some(Error::TooBig),
            ups_unused1 | ups_unused2 | ups_unused3 | ups_unused4 | ups_shut_up_gcc => {
                unreachable!("{:?} should never be used !", e)
            }
        }
    }
}

pub type Result<T> = core::result::Result<T, Error>;

// ---------------------------------------------------------------------------

/// A slice of memory owned by `flips`.
#[derive(Debug)]
pub struct FlipsMemory {
    mem: flips_sys::mem,
}

impl FlipsMemory {
    fn new(mem: flips_sys::mem) -> Self {
        Self { mem }
    }

    /// Copy the memory into a buffer managed by Rust.
    #[cfg_attr(feature = "_doc", doc(cfg(feature = "std")))]
    #[cfg(feature = "std")]
    pub fn into_bytes(self) -> Vec<u8> {
        self.as_ref().to_vec()
    }
}

impl AsRef<[u8]> for FlipsMemory {
    fn as_ref(&self) -> &[u8] {
        self.mem.as_ref()
    }
}

impl Deref for FlipsMemory {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl Drop for FlipsMemory {
    fn drop(&mut self) {
        unsafe {
            flips_sys::ips::ips_free(self.mem);
        }
    }
}

#[cfg_attr(feature = "_doc", doc(cfg(feature = "std")))]
#[cfg(feature = "std")]
impl Into<Vec<u8>> for FlipsMemory {
    fn into(self) -> Vec<u8> {
        self.into_bytes()
    }
}
