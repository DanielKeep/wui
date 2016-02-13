use std::io;
use std::mem;
use std::ptr;
use user32;
use winapi::*;
use ::last_error;
use ::traits::AsRaw;
use super::wnd::Wnd;

pub trait MsgExt: Sized {
    fn get(wnd: Option<&Wnd>, msg_filter: Option<(UINT, UINT)>) -> io::Result<Self>;
    fn post_quit(exit_code: INT);

    fn dispatch(&self) -> LRESULT;
    fn translate(&self) -> bool;
}

impl MsgExt for MSG {
    fn get(wnd: Option<&Wnd>, msg_filter: Option<(UINT, UINT)>) -> io::Result<MSG> {
        unsafe {
            let mut msg = mem::zeroed();
            let wnd = wnd.map(|v| v.as_raw()).unwrap_or(ptr::null_mut());
            let (msg_filter_min, msg_filter_max) = msg_filter.unwrap_or((0, 0));
            match user32::GetMessageW(&mut msg, wnd, msg_filter_min, msg_filter_max) {
                -1 => last_error(),
                _ => Ok(msg)
            }
        }
    }

    fn post_quit(exit_code: INT) {
        unsafe {
            user32::PostQuitMessage(exit_code)
        }
    }

    fn dispatch(&self) -> LRESULT {
        unsafe {
            user32::DispatchMessageW(self)
        }
    }

    fn translate(&self) -> bool {
        unsafe {
            match user32::TranslateMessage(self) {
                0 => false,
                _ => true
            }
        }
    }
}
