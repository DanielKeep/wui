use std::io;
use std::mem;
use std::ptr;
use user32;
use winapi::*;
use ::last_error;
use ::traits::AsRaw;
use ::util::WCString;
use super::wnd::Wnd;

pub trait MsgExt: Sized {
    fn get(wnd: Option<&Wnd>, msg_filter: Option<(UINT, UINT)>) -> io::Result<Self>;
    fn post_quit(exit_code: INT);

    fn dispatch(&self) -> LRESULT;
    fn is_dialog_message<Wnd>(&mut self, dlg: Wnd) -> bool where Wnd: AsRaw<Raw=HWND>;
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

    fn is_dialog_message<Wnd>(&mut self, dlg: Wnd) -> bool
    where Wnd: AsRaw<Raw=HWND> {
        unsafe {
            let dlg = dlg.as_raw();
            match user32::IsDialogMessageW(dlg, self) {
                0 => false,
                _ => true
            }
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

pub fn register_window_message(string: &str) -> io::Result<UINT> {
    unsafe {
        let string = WCString::from(string);
        let string = string.as_ptr();
        match user32::RegisterWindowMessageW(string) {
            0 => last_error(),
            v => Ok(v)
        }
    }
}
