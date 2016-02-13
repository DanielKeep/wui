use std::io;
use std::ptr;
use kernel32;
use winapi::*;
use wio::wide::ToWide;
use ::last_error;

pub fn get_module_handle(module_name: Option<&str>) -> io::Result<HMODULE> {
    unsafe {
        let module_name_wstr;
        let module_name = match module_name {
            Some(s) => {
                module_name_wstr = s.to_wide_null();
                module_name_wstr.as_ptr()
            },
            None => ptr::null(),
        };
        match kernel32::GetModuleHandleW(module_name) {
            v if v.is_null() => last_error(),
            v => Ok(v)
        }
    }
}
