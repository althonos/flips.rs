//! [![Star me](https://img.shields.io/github/stars/althonos/flips.rs.svg?style=social&label=Star&maxAge=3600)](https://github.com/althonos/flips.rs/stargazers)
//!
//! *Rust bindings to [Flips](https://github.com/Alcaro/Flips), the Floating IPS patcher.*
//!
//! [![TravisCI](https://img.shields.io/travis/com/althonos/flips.rs/master.svg?maxAge=600&style=flat-square)](https://travis-ci.com/althonos/flips.rs/branches)
//! [![Codecov](https://img.shields.io/codecov/c/gh/althonos/flips.rs/master.svg?style=flat-square&maxAge=600)](https://codecov.io/gh/althonos/flips.rs)
//! [![License](https://img.shields.io/badge/license-GPLv3-blue.svg?style=flat-square&maxAge=2678400)](https://choosealicense.com/licenses/gpl-3.0/)
//! [![Source](https://img.shields.io/badge/source-GitHub-303030.svg?maxAge=2678400&style=flat-square)](https://github.com/althonos/flips.rs)
//! [![Crate](https://img.shields.io/crates/v/flips.svg?maxAge=600&style=flat-square)](https://crates.io/crates/flips)
//! [![Documentation](https://img.shields.io/badge/docs.rs-latest-4d76ae.svg?maxAge=2678400&style=flat-square)](https://docs.rs/flips)
//! [![Changelog](https://img.shields.io/badge/keep%20a-changelog-8A0707.svg?maxAge=2678400&style=flat-square)](https://github.com/althonos/flips.rs/blob/master/CHANGELOG.md)
//! [![GitHub issues](https://img.shields.io/github/issues/althonos/flips.rs.svg?style=flat-square&maxAge=600)](https://github.com/althonos/flips.rs/issues)
//!
//! ## 📝 Features
//!
//! The following features are all enabled by default, but can be disabled and
//! cherry-picked using the `default-features = false` option in the `Cargo.toml`
//! manifest of your project:
//!
//! - **`std`**: compile against the Rust standard library, adding proper integration
//!   with [`std::error::Error`](https://doc.rust-lang.org/std/error/trait.Error.html)
//!   and [`Vec<u8>`](https://doc.rust-lang.org/std/vec/struct.Vec.html). Disable to
//!   compile in `no_std` mode.
//!
//! ## 📋 Changelog
//!
//! This project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html)
//! and provides a [changelog](https://github.com/althonos/flips.rs/blob/master/CHANGELOG.md)
//! in the [Keep a Changelog](http://keepachangelog.com/en/1.0.0/) format.
//!
//! ## 📜 License
//!
//! This library is provided under the
//! [GNU General Public License v3.0](https://choosealicense.com/licenses/gpl-3.0/),
//! since Flips itself is GPLv3 software.

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

/// The error type for this crate.
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
    /// Attempted to request size larger than [`libc::size_t`] for the target platform.
    ///
    /// [`libc::size_t`]: https://docs.rs/libc/latest/libc/type.size_t.html
    #[cfg_attr(feature = "std", error(display = "requested a size larger than `libc::size_t`"))]
    TooBig,
    /// Memory allocation failed.
    #[cfg_attr(feature = "std", error(display = "memory allocation failed"))]
    OutOfMem,
    /// Patch creation was canceled.
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

    /// Attempt to create an `Error` from a raw `bpserror`.
    fn from_bps(e: flips_sys::bps::bpserror) -> Option<Error> {
        use flips_sys::bps::bpserror::*;
        match e {
            bps_ok => None,
            bps_to_output => Some(Error::ToOutput),
            bps_not_this => Some(Error::NotThis),
            bps_broken => Some(Error::Invalid),
            bps_identical => Some(Error::Identical),
            bps_too_big => Some(Error::TooBig),
            bps_out_of_mem => Some(Error::OutOfMem),
            bps_canceled => Some(Error::Canceled),
            bps_shut_up_gcc | bps_io => {
                unreachable!("{:?} should never be used !", e)
            }
        }
    }
}

/// The result type for this crate.
pub type Result<T> = core::result::Result<T, Error>;

// ---------------------------------------------------------------------------

/// A slice of memory owned by `flips`.
///
/// You should never have to use this type directly, as each patch format
/// implements a different output struct that derefs to this type.
#[derive(Debug)]
pub struct FlipsMemory {
    mem: flips_sys::mem,
}

impl FlipsMemory {
    /// Create a new slice from a raw `flips_sys::mem` value.
    fn new(mem: flips_sys::mem) -> Self {
        Self { mem }
    }

    /// View the memory buffer as a raw slice of bytes.
    pub fn as_bytes(&self) -> &[u8] {
        self.mem.as_ref()
    }

    /// Copy the memory into a buffer managed by Rust.
    #[cfg_attr(feature = "_doc", doc(cfg(feature = "std")))]
    #[cfg(feature = "std")]
    pub fn to_bytes(&self) -> Vec<u8> {
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
        self.to_bytes()
    }
}
