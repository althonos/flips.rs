//! Bindings to `libups.h` to work with UPS patches.

#![allow(bad_style)]

use super::mem;

#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub enum upserror {
    /// Patch applied or created successfully.
    ups_ok,
    /// Unused, equivalent to `bps_to_output`.
    ups_unused1,
    /// Not the intended input file for this patch.
    ups_not_this,
    /// Not a UPS patch, or it's malformed somehow.
    ups_broken,
    /// Unused, equivalentto `bps_io`.
    ups_unused2,
    /// The input files are identical.
    ups_identical,
    /// Somehow, you're asking for something a `size_t` can't represent.
    ups_too_big,
    /// Unused, equivalent to `bps_out_of_mem`.
    ups_unused3,
    /// Unused, equivalent to `bps_canceled`.
    ups_unused4,
    /// Unused, just kill GCC warning.
    ups_shut_up_gcc,
}

#[link(name = "ups")]
extern "C" {
    /// Applies the UPS patch in `patch` to `in_` and stores it to `out`.
    ///
    /// Send the return value in out to `ups_free` when you're done with it.
    pub fn ups_apply(patch: mem, in_: mem, out: *mut mem) -> upserror;

    /// Creates an UPS patch that converts `source` to `target` and stores it to `patch`.
    ///
    /// Currently not implemented, will return an error everytime.
    pub fn ups_create(source: mem, target: mem, patch: *mut mem) -> upserror;

    /// Frees the memory returned in the output parameters of the above.
    ///
    /// Do not call it twice on the same input, nor on anything you got from
    /// anywhere else. `ups_free` is guaranteed to be equivalent to calling
    /// `free` from `<stdlib.h>` on `mem.ptr`.
    pub fn ups_free(mem: mem);
}
