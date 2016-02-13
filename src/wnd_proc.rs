use user32;
use winapi::*;

pub fn def_window_proc(wnd: HWND, message: UINT, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    unsafe {
        user32::DefWindowProcW(wnd, message, w_param, l_param)
    }
}
