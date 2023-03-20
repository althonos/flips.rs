use core::fmt::Debug;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;
use core::ops::Deref;
use core::ops::DerefMut;

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
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        let mut buffer = Self::default();
        for i in 0..buffer.buffer[..].len() {
            buffer.buffer[i] = u8::arbitrary(g);
        }
        buffer
    }
}
