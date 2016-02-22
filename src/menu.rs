use winapi::*;
use ::traits::AsRaw;

impl AsRaw for HMENU {
    type Raw = Self;
    fn as_raw(&self) -> Self {
        *self
    }
}
