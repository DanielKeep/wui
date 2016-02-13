use std::mem;
use winapi::*;
use ::traits::AsRaw;

pub struct Dc(HDC);

impl Dc {
    pub fn from_ref(dc: &HDC) -> &Dc {
        unsafe {
            mem::transmute(dc)
        }
    }
}

impl AsRaw for Dc {
    type Raw = HDC;
    fn as_raw(&self) -> Self::Raw {
        self.0
    }
}

impl AsRaw for HDC {
    type Raw = Self;
    fn as_raw(&self) -> Self::Raw {
        *self
    }
}
