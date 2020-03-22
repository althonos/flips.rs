extern crate flips_sys;

pub mod ips;
pub mod ups;
pub mod bps;

#[doc(inline)]
pub use self::bps::*;
#[doc(inline)]
pub use self::ips::*;
#[doc(inline)]
pub use self::ups::*;

use std::ops::Deref;

// ---------------------------------------------------------------------------

pub enum Error {
    /// Attempted to apply a patch not made for the input.
    NotThis,
    /// Attempted to apply a patch to the output ROM.
    ToOutput,
    /// The patch is invalid or malformed.
    Invalid,
    /// The patch is technically valid, but seems scrambled.
    Scrambled,
    Io,
    /// Attempted to create a patch from identical buffers.
    Identical,
    TooBig,
    OutOfMem,
    Canceled,
}

impl Error {
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

pub type Result<T> = std::result::Result<T, Error>;

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

    // Copy the memory into a buffer managed by Rust.
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

impl Into<Vec<u8>> for FlipsMemory {
    fn into(self) -> Vec<u8> {
        self.into_bytes()
    }
}
