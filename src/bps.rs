use core::ops::Deref;

use crate::Result;
use crate::Error;
use crate::FlipsMemory;

// ---------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq)]
pub struct BpsPatch<B: AsRef<[u8]>> {
    buffer: B,
}

impl<B: AsRef<[u8]>> BpsPatch<B> {
    /// Load a new BPS patch from an arbitrary sequence of bytes.
    ///
    /// The patch format is not checked, so this method will always succeed,
    /// but then [`apply`] may fail if the patch can't be read.
    pub fn new(buffer: B) -> Self {
        Self { buffer }
    }

    /// Apply the patch to a source.
    #[must_use]
    pub fn apply<S: AsRef<[u8]>>(&self, source: S) -> Result<BpsOutput> {
        let slice_p = self.buffer.as_ref();
        let slice_s = source.as_ref();
        let mut mem_m = flips_sys::mem::default();
        let mut mem_o = flips_sys::mem::default();

        let result = unsafe {
            let mem_i = flips_sys::mem::new(slice_s.as_ptr() as *mut _, slice_s.len());
            let mem_p = flips_sys::mem::new(slice_p.as_ptr() as *mut _, slice_p.len());
            flips_sys::bps::bps_apply(mem_p, mem_i, &mut mem_o as *mut _, &mut mem_m as *mut _, false)
        };

        match Error::from_bps(result) {
            None if mem_m.ptr.is_null() => Ok(BpsOutput::from(FlipsMemory::new(mem_o))),
            None => Ok(BpsOutput::with_metadata(FlipsMemory::new(mem_o), FlipsMemory::new(mem_m))),
            Some(error) => Err(error),
        }
    }
}

// ---------------------------------------------------------------------------

#[derive(Debug)]
pub struct BpsOutput {
    mem: FlipsMemory,
    metadata: Option<FlipsMemory>,
}

impl BpsOutput {
    fn with_metadata(output: FlipsMemory, metadata: FlipsMemory) -> Self {
        Self {
            mem: output,
            metadata: Some(metadata),
        }
    }
}

impl From<FlipsMemory> for BpsOutput {
    fn from(mem: FlipsMemory) -> Self {
        Self {
            mem,
            metadata: None
        }
    }
}

impl AsRef<[u8]> for BpsOutput {
    fn as_ref(&self) -> &[u8] {
        self.mem.as_ref()
    }
}

impl Deref for BpsOutput {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        self.mem.deref()
    }
}
