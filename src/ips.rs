use core::ops::Deref;

use crate::Result;
use crate::Error;
use crate::FlipsMemory;

// ---------------------------------------------------------------------------

/// A patch in the IPS format.
#[derive(Clone, Debug, PartialEq)]
pub struct IpsPatch<B: AsRef<[u8]>> {
    buffer: B,
}

impl<B: AsRef<[u8]>> IpsPatch<B> {
    /// Load a new IPS patch from an arbitrary sequence of bytes.
    ///
    /// The patch format is not checked, so this method will always succeed,
    /// but then [`apply`](#method.apply) or [`study`](#method.study) may
    /// fail if the patch can't be read.
    pub fn new(buffer: B) -> Self {
        Self { buffer }
    }

    /// Apply the patch to a source.
    #[must_use]
    pub fn apply<S: AsRef<[u8]>>(&self, source: S) -> Result<IpsOutput> {
        let slice_p = self.buffer.as_ref();
        let slice_s = source.as_ref();
        let mut mem_o = flips_sys::mem::default();

        let result = unsafe {
            let mem_i = flips_sys::mem::new(slice_s.as_ptr() as *mut _, slice_s.len());
            let mem_p = flips_sys::mem::new(slice_p.as_ptr() as *mut _, slice_p.len());
            flips_sys::ips::ips_apply(mem_p, mem_i, &mut mem_o as *mut _)
        };

        match Error::from_ips(result) {
            None => Ok(IpsOutput::from(FlipsMemory::new(mem_o))),
            Some(error) => Err(error),
        }
    }

    /// Create a study.
    #[must_use]
    pub fn study(self) -> Result<IpsStudy<B>> {
        let slice_p = self.buffer.as_ref();
        let mut study = flips_sys::ips::ipsstudy::default();

        let result = unsafe {
            let mem_p = flips_sys::mem::new(slice_p.as_ptr() as *mut _, slice_p.len());
            flips_sys::ips::ips_study(mem_p, &mut study as *mut _)
        };

        match Error::from_ips(result) {
            None => Ok(IpsStudy::new(self, study)),
            Some(error) => Err(error),
        }
    }
}

impl<B: AsRef<[u8]>> AsRef<[u8]> for IpsPatch<B> {
    fn as_ref(&self) -> &[u8] {
        self.buffer.as_ref()
    }
}

// ---------------------------------------------------------------------------

/// The output created by the application of an IPS patch.
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
    type Target = FlipsMemory;
    fn deref(&self) -> &Self::Target {
        &self.mem
    }
}

// ---------------------------------------------------------------------------

/// The result of a study over an IPS patch.
///
/// IPS studies allow to detect issues withing a patch before applying it to
/// a source buffer. It can check for the patch format, detecting if it is
/// invalid or corrupted.
#[derive(Clone, Debug)]
pub struct IpsStudy<B: AsRef<[u8]>> {
    patch: IpsPatch<B>,
    study: flips_sys::ips::ipsstudy,
}

impl<B: AsRef<[u8]>> IpsStudy<B> {
    fn new(patch: IpsPatch<B>, study: flips_sys::ips::ipsstudy) -> Self {
        Self {
            patch,
            study
        }
    }

    #[must_use]
    pub fn apply<S: AsRef<[u8]>>(&self, source: S) -> Result<IpsOutput> {
        // NB: we have to clone the study because `ips_apply_study` may
        //     change the study error if an error specific to the output
        //     is found. Not cloning would cause the error to be stored
        //     and to invalidate all further calls to `apply`.
        let mut study = self.study.clone();

        let slice_p = self.patch.buffer.as_ref();
        let slice_s = source.as_ref();
        let mut mem_o = flips_sys::mem::default();

        let result = unsafe {
            let mem_i = flips_sys::mem::new(slice_s.as_ptr() as *mut _, slice_s.len());
            let mem_p = flips_sys::mem::new(slice_p.as_ptr() as *mut _, slice_p.len());
            flips_sys::ips::ips_apply_study(mem_p, &mut study as *mut _, mem_i, &mut mem_o as *mut _)
        };

        match Error::from_ips(result) {
            None => Ok(IpsOutput::from(FlipsMemory::new(mem_o))),
            Some(error) => Err(error),
        }
    }
}

// ---------------------------------------------------------------------------

/// A builder to create an IPS patch.
#[derive(Clone, Debug, Default)]
pub struct IpsBuilder<S: AsRef<[u8]>, T: AsRef<[u8]>> {
    source: Option<S>,
    target: Option<T>
}

impl<S: AsRef<[u8]>, T: AsRef<[u8]>> IpsBuilder<S, T> {
    /// Create a new builder for an IPS patch.
    pub fn new() -> Self {
        Self {
            source: None,
            target: None,
        }
    }

    /// Set the source buffer for the patch.
    pub fn source(&mut self, source: S) -> &mut Self {
        self.source = Some(source);
        self
    }

    /// Set the target buffer for the patch.
    pub fn target(&mut self, target: T) -> &mut Self {
        self.target = Some(target);
        self
    }

    #[must_use]
    /// Build an IPS patch from `source` to `target`.
    ///
    /// # Error
    /// If either `source` or `target` was not given, this method will
    /// return [`Error::Canceled`](./enum.Error.html#variant.Canceled).
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
