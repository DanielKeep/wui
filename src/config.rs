pub mod system_parameters_info {
    use std::io;
    use std::mem;
    use conv::prelude::*;
    use winapi::*;
    use user32;
    use ::last_error;

    pub fn get_non_client_metrics() -> io::Result<NONCLIENTMETRICSW> {
        unsafe {
            let mut r: NONCLIENTMETRICSW = mem::uninitialized();
            let sz = mem::size_of_val(&r);
            let sz = try!(sz.value_as().or_else(|e| io_err!(e)));
            r.cbSize = sz;
            match user32::SystemParametersInfoW(SPI_GETNONCLIENTMETRICS, sz, mem::transmute(&mut r), 0) {
                0 => last_error(),
                _ => Ok(r)
            }
        }
    }
}