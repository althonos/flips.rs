//! Bindings to `libbps.h` to work with BPS patches.

#![allow(bad_style)]

use super::mem;

#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub enum bpserror {
    /// Patch applied or created successfully.
    bps_ok,
    /// You attempted to apply a patch to its output.
    bps_to_output,
    /// This is not the intended input file for this patch.
    bps_not_this,
    /// This is not a BPS patch, or it's malformed somehow.
    bps_broken,
    /// The patch could not be read.
    bps_io,
    /// The input files are identical.
    bps_identical,
    /// Somehow, you're asking for something a `size_t` can't represent.
    bps_too_big,
    /// Memory allocation failure.
    bps_out_of_mem,
    /// Patch creation was canceled.
    bps_canceled,
    /// Unused, just kill GCC warning.
    bps_shut_up_gcc,
}

#[link(name="bps")]
extern "C" {

    /// Applies the BPS patch to the ROM in `in_` and puts it in `out`.
    ///
    /// Metadata, if present and requested, (`metadata` is not NULL), is also
    /// returned. Send both output to `bps_free` when you're done with them.
    ///
    /// If `accept_wrong_input` is true, it may return `bps_to_output` or
    /// `bps_not_this`, while putting non-NULL in `out`/`metadata`.
    pub fn bps_apply(
        patch: mem,
        in_: mem,
        out: *mut mem,
        metadata: *mut mem,
        accept_wrong_input: bool,
    ) -> bpserror;


    /// Creates a BPS patch that converts `source` to `target` and stores it to `patch`.
    ///
    /// It is safe to give `{NULL, 0}` as `metadata`.
    pub fn bps_create_linear(
        source: mem,
        target: mem,
        metadata: mem,
        patch: *mut mem
    ) -> bpserror;
}



#[cfg(test)]
mod tests {

    use std::ops::Deref;

    use super::mem;
    use super::bpserror;
    use crate::test_utils::ArbitraryBuffer;

    #[quickcheck_macros::quickcheck]
    fn check_create_and_apply(mut source: ArbitraryBuffer, mut target: ArbitraryBuffer) -> bool {
        if source == target {
            return true;
        }

        unsafe {
            // create patch
            let mut mem_patch = mem::default();
            let result = super::bps_create_linear(source.to_mem(), target.to_mem(), mem::default(), &mut mem_patch as *mut mem);
            assert_eq!(result, bpserror::bps_ok, "could not create patch");

            // apply patch
            let mut mem_out = mem::default();
            let result = super::bps_apply(mem_patch, source.to_mem(), &mut mem_out as *mut mem, &mut mem::default() as *mut mem, false);
            assert_eq!(result, bpserror::bps_ok, "could not apply patch");

            // check
            mem_out.as_ref() == target.deref()
        }
    }

    #[quickcheck_macros::quickcheck]
    fn check_create_identical(mut source: ArbitraryBuffer) -> bool {
        unsafe {
            let mut mem_patch = std::mem::MaybeUninit::uninit().assume_init();
            let result = super::bps_create_linear(source.to_mem(), source.to_mem(), mem::default(), &mut mem_patch as *mut _);
            result == bpserror::bps_identical
        }
    }

    #[quickcheck_macros::quickcheck]
    fn check_create_equal(mut source: ArbitraryBuffer) -> bool {
        let mut target = source.clone();
        unsafe {
            let mut mem_patch = std::mem::MaybeUninit::uninit().assume_init();
            let result = super::bps_create_linear(source.to_mem(), target.to_mem(), mem::default(), &mut mem_patch as *mut _);
            result == bpserror::bps_identical
        }
    }
}
