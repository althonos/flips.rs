#![allow(bad_style)]

use super::mem;

#[repr(C)]
#[derive(Debug)]
pub enum upserror {
    ups_ok,
    ups_unused1,
    ups_not_this,
    ups_broken,
    ups_unused2,
    ups_identical,
    ups_too_big,
    ups_unused3,
    ups_unused4,
    ups_shut_up_gcc,
}

#[link(name = "ups")]
extern "C" {
    // Applies the UPS patch in [patch, patchlen] to [in, inlen] and stores it to [out, outlen]. Send the
    // return value in out to ups_free when you're done with it.
    pub fn ups_apply(patch: mem, in_: mem, out: *mut mem) -> upserror;

    //Creates an UPS patch that converts source to target and stores it to patch. (Not implemented.)
    pub fn ups_create(source: mem, target: mem, patch: *mut mem) -> upserror;

    //Frees the memory returned in the output parameters of the above. Do not call it twice on the same
    //  input, nor on anything you got from anywhere else. ups_free is guaranteed to be equivalent to
    //  calling stdlib.h's free() on mem.ptr.
    pub fn ups_free(mem: mem);
}
