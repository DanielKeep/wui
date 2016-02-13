use std::io;
use std::mem;
use user32;
use winapi::*;
use ::last_error;
use ::traits::AsRaw;
use super::dc::Dc;

pub struct WndPaint(HWND, PAINTSTRUCT);

impl WndPaint {
    pub fn begin_paint<Wnd: AsRaw<Raw=HWND>>(wnd: Wnd) -> io::Result<Self> {
        unsafe {
            let mut ps = mem::uninitialized();
            let wnd = wnd.as_raw();
            match user32::BeginPaint(wnd, &mut ps) {
                v if v.is_null() => last_error(),
                _ => Ok(WndPaint(wnd, ps))
            }
        }
    }

    pub fn dc(&self) -> &Dc {
        Dc::from_ref(&self.1.hdc)
    }
}

impl Drop for WndPaint {
    fn drop(&mut self) {
        unsafe {
            user32::EndPaint(self.0, &self.1);
        }
    }
}
