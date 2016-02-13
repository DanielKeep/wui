use std::error::Error;
use std::mem;
use wio::wide::ToWide;

pub struct WCString(Vec<u16>);

impl WCString {
    pub fn as_ptr(&self) -> *const u16 {
        self.0.as_ptr()
    }
}

impl<T> From<T> for WCString where T: ToWide {
    fn from(v: T) -> Self {
        WCString(v.to_wide_null())
    }
}

pub trait TryDrop: Sized {
    type Err: Error;

    fn try_drop(mut self) -> Result<(), Self::Err> {
        unsafe {
            try!(self.try_drop_inner());
            mem::forget(self);
            Ok(())
        }
    }

    unsafe fn try_drop_inner(&mut self) -> Result<(), Self::Err>;
}
