use std::io;
use conv::prelude::*;
use winapi::*;
use wio::wide::ToWide;
use ::last_error;
use ::traits::AsRaw;

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
            0 => last_error(),
            _ => Ok(())
        }
    }
}
