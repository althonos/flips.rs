extern crate flips_sys;

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

// ---------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq)]
pub struct IpsPatch<B: AsRef<[u8]>> {
    buffer: B,
}

impl<B: AsRef<[u8]>> IpsPatch<B> {
    /// Load a new IPS patch from an arbitrary sequence of bytes.
    ///
    /// The patch format is not checked, so this method will always succeed,
    /// but then `IpsPatch.apply` may fail if the patch can't be read.
    pub fn new(buffer: B) -> Self {
        Self { buffer }
    }

    /// Apply the patch to a source.
    pub fn apply<S: AsRef<[u8]>>(&self, source: S) -> Result<FlipsMemory> {
        let slice_p = self.buffer.as_ref();
        let slice_s = source.as_ref();
        let mut mem_o = flips_sys::mem::default();

        let result = unsafe {
            let mem_i = flips_sys::mem::new(slice_s.as_ptr() as *mut _, slice_s.len());
            let mem_p = flips_sys::mem::new(slice_p.as_ptr() as *mut _, slice_p.len());
            flips_sys::ips::ips_apply(mem_p, mem_i, &mut mem_o as *mut _)
        };

        match Error::from_ips(result) {
            None => Ok(FlipsMemory::new(mem_o)),
            Some(error) => Err(error),
        }
    }
}

impl IpsPatch<FlipsMemory> {
    /// Create a new patch.
    pub fn create<S: AsRef<[u8]>, T: AsRef<[u8]>>(source: S, target: T) -> Result<Self> {
        let slice_s = source.as_ref();
        let slice_t = target.as_ref();
        let mut mem_patch = flips_sys::mem::default();

        let result = unsafe {
            let mem_s = flips_sys::mem::new(slice_s.as_ptr() as *mut _, slice_s.len());
            let mem_t = flips_sys::mem::new(slice_t.as_ptr() as *mut _, slice_t.len());
            flips_sys::ips::ips_create(mem_s, mem_t, &mut mem_patch as *mut _)
        };

        match Error::from_ips(result) {
            None => Ok(IpsPatch::new(FlipsMemory::new(mem_patch))),
            Some(error) => Err(error),
        }
    }
}

// ---------------------------------------------------------------------------

pub struct UpsPatch<B: AsRef<[u8]>> {
    buffer: B,
}

impl<B: AsRef<[u8]>> UpsPatch<B> {
    /// Load a new UPS patch from an arbitrary sequence of bytes.
    pub fn new(buffer: B) -> Self {
        Self { buffer }
    }

    /// Apply the patch to a source.
    pub fn apply<S: AsRef<[u8]>>(&self, source: S) -> Result<FlipsMemory> {
        let slice_p = self.buffer.as_ref();
        let slice_s = source.as_ref();
        let mut mem_o = flips_sys::mem::default();

        let result = unsafe {
            let mem_i = flips_sys::mem::new(slice_s.as_ptr() as *mut _, slice_s.len());
            let mem_p = flips_sys::mem::new(slice_p.as_ptr() as *mut _, slice_p.len());
            flips_sys::ups::ups_apply(mem_p, mem_i, &mut mem_o as *mut _)
        };

        match Error::from_ups(result) {
            None => Ok(FlipsMemory::new(mem_o)),
            Some(error) => Err(error),
        }
    }
}
