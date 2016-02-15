use user32;
use winapi::*;

pub fn def_window_proc(wnd: HWND, message: UINT, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    unsafe {
        user32::DefWindowProcW(wnd, message, w_param, l_param)
    }
}

#[derive(Debug)]
pub enum WmCommand {
    Menu { id: u16 },
    Accelerator { id: u16 },
    Control { code: u16, id: u16, ctl_wnd: HWND },
}

pub fn wm_command(w_param: WPARAM, l_param: LPARAM) -> WmCommand {
    use self::WmCommand::*;
    let w_param = w_param as u32;
    match (HIWORD(w_param), l_param) {
        /*
        We test *both* `w_param` and `l_param` here because, despite what MSDN suggests, there are *two* cases where `HIWORD(w_param)` is zero: menu commands and `BN_CLICKED`.  The distinction is that a menu command should have a `l_param` of 0, whilst a button click shouldn't.
        */
        (0, 0) => Menu { id: LOWORD(w_param) },
        (1, _) => Accelerator { id: LOWORD(w_param) },
        (n, _) => Control {
            code: n,
            id: LOWORD(w_param),
            ctl_wnd: l_param as HWND,
        }
    }
}
