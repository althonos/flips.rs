#![allow(bad_style)]

use super::mem;

#[repr(C)]
#[derive(Debug, PartialEq)]
pub enum bpserror {
    bps_ok,
    bps_to_output,
    bps_not_this,
    bps_broken,
    bps_io,
    bps_identical,
    bps_too_big,
    bps_out_of_mem,
    bps_canceled,
    bps_shut_up_gcc,
}

#[link(name="bps")]
extern "C" {
    pub fn bps_apply(
        patch: mem,
        in_: mem,
        out: *mut mem,
        metadata: *mut mem,
        accept_wrong_input: bool,
    ) -> bpserror;


    //Creates a BPS patch that converts source to target and stores it to patch. It is safe to give
    //  {NULL,0} as metadata.
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
