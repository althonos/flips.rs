#![allow(bad_style)]

use super::mem;

#[repr(C)]
#[derive(Debug, PartialEq)]
pub enum ipserror {
    ips_ok,
    ips_notthis,
    ips_thisout,
    ips_scrambled,
    ips_invalid,
    ips_16MB,
    ips_identical,
    ips_shut_up_gcc,
}

#[repr(C)]
pub struct ipsstudy { _private: [u8; 0] }

#[link(name="ips")]
extern "C" {
    /// Applies the IPS patch in [patch, patchlen] to [in, inlen] and stores it to [out, outlen]. Send the
    /// return value in out to ips_free when you're done with it.
    pub fn ips_apply(patch: mem, in_: mem, out: *mut mem) -> ipserror;

    // Creates an IPS patch that converts source to target and stores it to patch.
    pub fn ips_create(source: mem, target: mem, patch: *mut mem) -> ipserror;

    //Frees the memory returned in the output parameters of the above. Do not call it twice on the same
    //  input, nor on anything you got from anywhere else. ips_free is guaranteed to be equivalent to
    //  calling stdlib.h's free() on mem.ptr.
    pub fn ips_free(mem: mem);

    //ips_study allows you to detect most patching errors without applying it to a ROM, or even a ROM to
    //  apply it to. ips_apply calls ips_study and ips_apply_study, so if you call ips_study yourself,
    //  it's recommended to call ips_apply_study to not redo the calculation. ips_free is still
    //  required.
    pub fn ips_study(patch: mem, study: *mut ipsstudy) -> ipserror;
    pub fn ips_apply_study(patch: mem, study: *mut ipsstudy, in_: mem, out: *mut mem) -> ipserror;
}

#[cfg(test)]
mod tests {

    use std::ops::Deref;

    use super::mem;
    use super::ipserror;
    use crate::test_utils::ArbitraryBuffer;

    #[quickcheck_macros::quickcheck]
    fn check_create_and_apply(mut source: ArbitraryBuffer, mut target: ArbitraryBuffer) -> bool {
        if source == target {
            return true;
        }

        unsafe {
            // create patch
            let mut mem_patch = mem::default();
            let result = super::ips_create(source.to_mem(), target.to_mem(), &mut mem_patch as *mut mem);
            assert_eq!(result, ipserror::ips_ok, "could not create patch");

            // apply patch
            let mut mem_out = mem::default();
            let result = super::ips_apply(mem_patch, source.to_mem(), &mut mem_out as *mut mem);
            assert_eq!(result, ipserror::ips_ok, "could not apply patch");

            // check
            mem_out.as_ref() == target.deref()
        }
    }

    #[quickcheck_macros::quickcheck]
    fn check_create_identical(mut source: ArbitraryBuffer) -> bool {
        unsafe {
            let mut mem_patch = std::mem::MaybeUninit::uninit().assume_init();
            let result = super::ips_create(source.to_mem(), source.to_mem(), &mut mem_patch as *mut _);
            result == ipserror::ips_identical
        }
    }

    #[quickcheck_macros::quickcheck]
    fn check_create_equal(mut source: ArbitraryBuffer) -> bool {
        let mut target = source.clone();
        unsafe {
            let mut mem_patch = std::mem::MaybeUninit::uninit().assume_init();
            let result = super::ips_create(source.to_mem(), target.to_mem(), &mut mem_patch as *mut _);
            result == ipserror::ips_identical
        }
    }
}
