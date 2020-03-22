use core::ops::Deref;

use crate::Result;
use crate::Error;
use crate::FlipsMemory;

// ---------------------------------------------------------------------------

/// A patch in the UPS format.
#[derive(Clone, Debug, PartialEq)]
pub struct UpsPatch<B: AsRef<[u8]>> {
    buffer: B,
}

impl<B: AsRef<[u8]>> UpsPatch<B> {
    /// Load a new UPS patch from an arbitrary sequence of bytes.
    pub fn new(buffer: B) -> Self {
        Self { buffer }
    }

    /// Apply the patch to a source.
    ///
    /// # Warning
    /// Applying a UPS patch to its output will not return any error, but
    /// generate the input file back again (this is known as *backwargs*
    /// application in `libups`).
    pub fn apply<S: AsRef<[u8]>>(&self, source: S) -> Result<UpsOutput> {
        let slice_p = self.buffer.as_ref();
        let slice_s = source.as_ref();
        let mut mem_o = flips_sys::mem::default();

        let result = unsafe {
            let mem_i = flips_sys::mem::new(slice_s.as_ptr() as *mut _, slice_s.len());
            let mem_p = flips_sys::mem::new(slice_p.as_ptr() as *mut _, slice_p.len());
            flips_sys::ups::ups_apply(mem_p, mem_i, &mut mem_o as *mut _)
        };

        match Error::from_ups(result) {
            None => Ok(UpsOutput::from(FlipsMemory::new(mem_o))),
            Some(error) => Err(error),
        }
    }
}

// ---------------------------------------------------------------------------

/// The output created by the application of a UPS patch.
#[derive(Debug)]
pub struct UpsOutput {
    mem: FlipsMemory,
}

impl From<FlipsMemory> for UpsOutput {
    fn from(mem: FlipsMemory) -> Self {
        Self { mem }
    }
}

impl AsRef<[u8]> for UpsOutput {
    fn as_ref(&self) -> &[u8] {
        self.mem.as_ref()
    }
}

impl Deref for UpsOutput {
    type Target = FlipsMemory;
    fn deref(&self) -> &Self::Target {
        &self.mem
    }
}
