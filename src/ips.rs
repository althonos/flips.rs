use std::ops::Deref;

use crate::Result;
use crate::Error;
use crate::FlipsMemory;

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

// ---------------------------------------------------------------------------

#[derive(Debug)]
pub struct IpsOutput {
    mem: FlipsMemory,
}

impl From<FlipsMemory> for IpsOutput {
    fn from(mem: FlipsMemory) -> Self {
        Self { mem }
    }
}

impl AsRef<[u8]> for IpsOutput {
    fn as_ref(&self) -> &[u8] {
        self.mem.as_ref()
    }
}

impl Deref for IpsOutput {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        self.mem.deref()
    }
}

// ---------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct IpsBuilder<S: AsRef<[u8]>, T: AsRef<[u8]>> {
    source: Option<S>,
    target: Option<T>
}

impl<S: AsRef<[u8]>, T: AsRef<[u8]>> IpsBuilder<S, T> {
    pub fn new() -> Self {
        Self {
            source: None,
            target: None,
        }
    }

    pub fn source(&mut self, source: S) -> &mut Self {
        self.source = Some(source);
        self
    }

    pub fn target(&mut self, target: T) -> &mut Self {
        self.target = Some(target);
        self
    }

    #[must_use]
    pub fn build(&mut self) -> Result<IpsPatch<FlipsMemory>> {
        if self.source.is_none() || self.target.is_none() {
            return Err(Error::Canceled);
        }

        let (source, target) = (self.source.take().unwrap(), self.target.take().unwrap());
        let (slice_s, slice_t) = (source.as_ref(), target.as_ref());
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
