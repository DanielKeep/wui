use std::io;
use conv::prelude::*;
use gdi32;
use winapi::*;
use wio::wide::ToWide;
use ::other_error;
use ::traits::{AsRaw, FromRaw};
use ::util::TryDrop;

#[derive(Debug)]
pub struct Font(HFONT);

impl Font {
    pub fn create(font: &LOGFONTW) -> io::Result<Self> {
        unsafe {
            match gdi32::CreateFontIndirectW(font) {
                v if v.is_null() => other_error("CreateFontIndirectW failed"),
                v => Ok(Font(v))
            }
        }
    }
}

impl AsRaw for Font {
    type Raw = HFONT;
    fn as_raw(&self) -> Self::Raw {
        self.0
    }
}

impl AsRaw for HFONT {
    type Raw = Self;
    fn as_raw(&self) -> Self::Raw {
        *self
    }
}

impl Drop for Font {
    fn drop(&mut self) {
        unsafe { self.try_drop_inner().unwrap() }
    }
}

impl FromRaw for Font {
    unsafe fn from_raw(raw: HFONT) -> Font {
        Font(raw)
    }
}

impl TryDrop for Font {
    type Err = io::Error;
    unsafe fn try_drop_inner(&mut self) -> Result<(), Self::Err> {
        match gdi32::DeleteObject(self.0 as HGDIOBJ) {
            0 => other_error(&format!("DeleteObject on HFONT {:p} failed", self.0)),
            _ => Ok(())
        }
    }
}

extern "system" {
    fn TextOutW(hdc: HDC, nXStart: INT, nYStart: INT, lpString: LPCWSTR, cchString: INT) -> BOOL;
}

pub fn text_out<Dc>(dc: Dc, x_start: i32, y_start: i32, string: &str) -> io::Result<()>
where Dc: AsRaw<Raw=HDC> {
    unsafe {
        let dc = dc.as_raw();
        let string = string.to_wide();
        let ch_string = string.len().value_as::<INT>().unwrap_or_saturate();
        let string = string.as_ptr();
        match TextOutW(dc, x_start, y_start, string, ch_string) {
            0 => other_error("TextOutW failed"),
            _ => Ok(())
        }
    }
}
