extern crate winapi;
extern crate wui;

use std::io;
use winapi::*;
use wui::*;

fn main() {
    use wui::message_box_type as mbt;

    match try_main() {
        Ok(()) => (),
        Err(err) => {
            let msg = format!("Error: {}", err);
            let _ = message_box(None, &msg, None, Some(mbt::Ok | mbt::IconError));
        }
    }
}

fn try_main() -> io::Result<()> {
    let wnd_class = try!(WndClass::new()
        .class_name("Hello")
        .instance(try!(get_module_handle(None)))
        .wnd_proc(wnd_proc)
        .register());

    let wnd = try!(Wnd::new()
        .class_name(&wnd_class)
        .window_name("Hello")
        .style(wnd_style::OverlappedWindow)
        .create());

    wnd.show(Show::ShowDefault);
    try!(wnd.update());

    loop {
        match try!(MSG::get(None, None)) {
            MSG { message: WM_QUIT, wParam: code, .. } => {
                ::std::process::exit(code as i32);
            },
            msg => {
                msg.translate();
                msg.dispatch();
            }
        }
    }
}

unsafe extern "system" fn wnd_proc(wnd: HWND, message: UINT, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    match message {
        winapi::WM_DESTROY => {
            MSG::post_quit(0);
            0
        },
        message => {
            def_window_proc(wnd, message, w_param, l_param)
        }
    }
}
