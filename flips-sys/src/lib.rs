#![cfg_attr(not(test), no_std)]
#![allow(bad_style)]

extern crate libc;

#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
extern crate quickcheck_macros;

pub mod bps;
pub mod ips;
pub mod ups;


#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct mem {
    pub ptr: *mut u8,
    pub len: libc::size_t,
}

impl mem {
    pub fn new(ptr: *mut u8, len: libc::size_t) -> Self {
        Self { ptr, len }
    }
}

impl Default for mem {
    fn default() -> Self {
        Self::new(core::ptr::null_mut(), 0)
    }
}

impl AsRef<[u8]> for mem {
    fn as_ref(&self) -> &[u8] {
        unsafe {
            core::slice::from_raw_parts(self.ptr as *const _, self.len)
        }
    }
}


#[cfg(test)]
pub mod test_utils {

    use std::fmt::Debug;
    use std::fmt::Formatter;
    use std::fmt::Result as FmtResult;
    use std::ops::Deref;
    use std::ops::DerefMut;

    #[derive(Clone)]
    pub struct ArbitraryBuffer {
        buffer: [u8; 4096],
    }

    impl ArbitraryBuffer {
        pub fn to_mem(&mut self) -> super::mem {
            super::mem::new(self.deref_mut().as_mut_ptr(), self.buffer[..].len())
        }
    }

    impl Debug for ArbitraryBuffer {
        fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
            self.buffer[..].fmt(fmt)
        }
    }

    impl Default for ArbitraryBuffer {
        fn default() -> Self {
            Self {
                buffer: [0; 4096],
            }
        }
    }

    impl Deref for ArbitraryBuffer {
        type Target = [u8];
        fn deref(&self) -> &Self::Target {
            &self.buffer[..]
        }
    }

    impl DerefMut for ArbitraryBuffer {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.buffer[..]
        }
    }

    impl PartialEq for ArbitraryBuffer {
        fn eq(&self, other: &Self) -> bool {
            (0..self.buffer[..].len()).all(|i| self.buffer[i] == other.buffer[i])
        }
    }

    impl quickcheck::Arbitrary for ArbitraryBuffer {
        fn arbitrary<G: quickcheck::Gen>(g: &mut G) -> Self {
            let mut buffer = Self::default();
            for i in 0..buffer.buffer[..].len() {
                buffer.buffer[i] = u8::arbitrary(g);
            }
            buffer
        }
    }
}
