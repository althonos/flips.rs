use core::ops::Deref;

use crate::Result;
use crate::Error;
use crate::FlipsMemory;

// ---------------------------------------------------------------------------

/// A patch in the BPS format.
#[derive(Clone, Debug, PartialEq)]
pub struct BpsPatch<B: AsRef<[u8]>> {
    buffer: B,
}

impl<B: AsRef<[u8]>> BpsPatch<B> {
    /// Load a new BPS patch from an arbitrary sequence of bytes.
    ///
    /// The patch format is not checked, so this method will always succeed,
    /// but then [`apply`](#method.apply) may fail if the patch can't be read.
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

/// The output created by the application of a BPS patch.
#[derive(Debug)]
pub struct BpsOutput {
    mem: FlipsMemory,
    metadata: Option<FlipsMemory>,
}

impl BpsOutput {
    /// Create a `BpsOutput` using some metadata.
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
    type Target = FlipsMemory;
    fn deref(&self) -> &Self::Target {
        &self.mem
    }
}

// ---------------------------------------------------------------------------

/// A builder to create a BPS patch.
///
/// This type is as generic as possible, making it easy to use any type that
/// can be viewed as a slice of bytes.
///
/// # Example
/// ```rust
/// let patch = flips::BpsLinearBuilder::new()
///     .source(&b"some source bytes"[..])
///     .target(&b"some target bytes"[..])
///     .build()
///     .expect("could not create patch");
/// ```
#[derive(Clone, Debug, Default)]
pub struct BpsLinearBuilder<S: AsRef<[u8]>, T: AsRef<[u8]>, M: AsRef<[u8]> = &'static [u8]> {
    source: Option<S>,
    target: Option<T>,
    metadata: Option<M>,
}

impl<S: AsRef<[u8]>, T: AsRef<[u8]>> BpsLinearBuilder<S, T, &'static [u8]> {
    /// Create a new builder for an BPS patch.
    pub fn new() -> Self {
        Self {
            source: None,
            target: None,
            metadata: None,
        }
    }

    /// Set the metadata buffer for the patch, if any.
    pub fn metadata<M: AsRef<[u8]>, B: Into<Option<M>>>(&mut self, buffer: B) -> BpsLinearBuilder<S, T, M> {
        BpsLinearBuilder {
            source: self.source.take(),
            target: self.target.take(),
            metadata: buffer.into(),
        }
    }
}

impl<S: AsRef<[u8]>, T: AsRef<[u8]>, M: AsRef<[u8]>> BpsLinearBuilder<S, T, M> {
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
    /// Build an BPS patch from `source` to `target` with `metadata` if any.
    ///
    /// # Error
    /// If either `source` or `target` was not given, this method will
    /// return [`Error::Canceled`](./enum.Error.html#variant.Canceled).
    pub fn build(&mut self) -> Result<BpsPatch<FlipsMemory>> {
        if self.source.is_none() || self.target.is_none() {
            return Err(Error::Canceled);
        }

        let (source, target) = (self.source.take().unwrap(), self.target.take().unwrap());
        let (slice_s, slice_t) = (source.as_ref(), target.as_ref());
        let mut mem_patch = flips_sys::mem::default();

        let result = unsafe {
            let mem_metadata = match self.metadata.take() {
                Some(m) => flips_sys::mem::new(m.as_ref().as_ptr() as *mut _, m.as_ref().len()),
                None => flips_sys::mem::default(),
            };
            let mem_s = flips_sys::mem::new(slice_s.as_ptr() as *mut _, slice_s.len());
            let mem_t = flips_sys::mem::new(slice_t.as_ptr() as *mut _, slice_t.len());
            flips_sys::bps::bps_create_linear(
                mem_s,
                mem_t,
                mem_metadata,
                &mut mem_patch as *mut _
            )
        };

        match Error::from_bps(result) {
            None => Ok(BpsPatch::new(FlipsMemory::new(mem_patch))),
            Some(error) => Err(error),
        }
    }
}

// --

/// A builder to create a BPS patch.
///
/// This type is as generic as possible, making it easy to use any type that
/// can be viewed as a slice of bytes.
///
/// # Example
/// ```rust
/// let patch = flips::BpsLinearBuilder::new()
///     .source(&b"some source bytes"[..])
///     .target(&b"some target bytes"[..])
///     .build()
///     .expect("could not create patch");
/// ```
#[derive(Clone, Debug, Default)]
pub struct BpsDeltaBuilder<S: AsRef<[u8]>, T: AsRef<[u8]>, M: AsRef<[u8]> = &'static [u8]> {
    source: Option<S>,
    target: Option<T>,
    metadata: Option<M>,
}

impl<S: AsRef<[u8]>, T: AsRef<[u8]>> BpsDeltaBuilder<S, T, &'static [u8]> {
    /// Create a new builder for an BPS patch.
    pub fn new() -> Self {
        Self {
            source: None,
            target: None,
            metadata: None,
        }
    }

    /// Set the metadata buffer for the patch, if any.
    pub fn metadata<M: AsRef<[u8]>, B: Into<Option<M>>>(&mut self, buffer: B) -> BpsDeltaBuilder<S, T, M> {
        BpsDeltaBuilder {
            source: self.source.take(),
            target: self.target.take(),
            metadata: buffer.into(),
        }
    }
}

impl<S: AsRef<[u8]>, T: AsRef<[u8]>, M: AsRef<[u8]>> BpsDeltaBuilder<S, T, M> {
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
    /// Build an BPS patch from `source` to `target` with `metadata` if any.
    ///
    /// # Error
    /// If either `source` or `target` was not given, this method will
    /// return [`Error::Canceled`](./enum.Error.html#variant.Canceled).
    pub fn build(&mut self) -> Result<BpsPatch<FlipsMemory>> {
        if self.source.is_none() || self.target.is_none() {
            return Err(Error::Canceled);
        }

        let (source, target) = (self.source.take().unwrap(), self.target.take().unwrap());
        let (slice_s, slice_t) = (source.as_ref(), target.as_ref());
        let mut mem_patch = flips_sys::mem::default();

        let result = unsafe {
            let mem_metadata = match self.metadata.take() {
                Some(m) => flips_sys::mem::new(m.as_ref().as_ptr() as *mut _, m.as_ref().len()),
                None => flips_sys::mem::default(),
            };
            let mem_s = flips_sys::mem::new(slice_s.as_ptr() as *mut _, slice_s.len());
            let mem_t = flips_sys::mem::new(slice_t.as_ptr() as *mut _, slice_t.len());
            flips_sys::bps::bps_create_delta_inmem(
                mem_s,
                mem_t,
                mem_metadata,
                &mut mem_patch as *mut _,
                core::ptr::null(),
                core::ptr::null(),
                false,
            )
        };

        match Error::from_bps(result) {
            None => Ok(BpsPatch::new(FlipsMemory::new(mem_patch))),
            Some(error) => Err(error),
        }
    }
}
