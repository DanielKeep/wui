#![feature(recover)]
#![feature(type_ascription)]

#[macro_use] extern crate wui;
extern crate winapi;

use std::io;
use winapi::*;
use wui::*;

const BTN_HELLO_ID: u16 = 101;

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
    use wui::button_style as bs;
    use wui::wnd_style as ws;

    let wnd_class = try!(WndClass::new()
        .class_name("Hello")
        .instance(try!(get_module_handle(None)))
        .icon(try!(Icon::load(None, IDI_APPLICATION)))
        .cursor(try!(Cursor::load(None, IDC_ARROW)))
        .background(try!(Brush::get_sys_color(Color::BtnFace)))
        .wnd_proc(wnd_proc)
        .register());

    let wnd = try!(Wnd::new()
        .class_name(&wnd_class)
        .window_name("Hello")
        .style(ws::OverlappedWindow)
        .create());

    let _btn = try!(Wnd::new()
        .class_name("BUTTON")
        .window_name("Hello")
        .style(ws::TabStop | ws::Visible | ws::Child | bs::DefPushButton)
        .x(10).y(10)
        .width(100).height(100)
        .wnd_parent(&wnd)
        .button_id(BTN_HELLO_ID)
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

fn try_wnd_proc(wnd: HWND, message: UINT, w_param: WPARAM, l_param: LPARAM) -> io::Result<LRESULT> {
    trace_message(wnd, message, w_param, l_param);
    match message {
        WM_COMMAND => {
            use wui::WmCommand::*;
            match wm_command(w_param, l_param) {
                Control { code: BN_CLICKED, id: BTN_HELLO_ID, .. } => {
                    try!(message_box(Some(wnd), "Hello, World!", Some("Hello"), None));
                    Ok(0)
                },
                _ => Ok(def_window_proc(wnd, message, w_param, l_param))
            }
        },
        WM_DESTROY => {
            MSG::post_quit(0);
            Ok(0)
        },
        WM_RBUTTONUP => {
            panic!("Kaboom!");
        },
        message => {
            Ok(def_window_proc(wnd, message, w_param, l_param))
        }
    }
}

#[cfg(feature="trace-messages")]
fn trace_message(wnd: HWND, message: UINT, w_param: WPARAM, l_param: LPARAM) {
    println!("wnd_proc(wnd: {:?}, message: {:?}, w_param: {:?}, l_param: {:?})", wnd, FormatMsg(message), w_param, l_param);
}

#[cfg(not(feature="trace-messages"))]
fn trace_message(_: HWND, _: UINT, _: WPARAM, _: LPARAM) {}

unsafe extern "system" fn wnd_proc(wnd: HWND, message: UINT, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    match ::std::panic::recover(move || {
        try_wnd_proc(wnd, message, w_param, l_param)
    }) {
        Ok(Ok(res)) => res,
        Ok(Err(err)) => wui_abort!(
            "\
                Unhandled error: {}\r\n\
                \r\n\
                wnd: 0x{:p}\r\n\
                message: 0x{:x}\r\n\
                w_param: 0x{:x}\r\n\
                l_param: 0x{:x}\
            ",
            err, wnd, message, w_param, l_param),
        Err(err) => {
            let msg = if let Some(err) = err.downcast_ref::<&'static str>() {
                String::from(*err)
            } else if let Some(err) = err.downcast_ref::<String>() {
                (*err).clone()
            } else if let Some(err) = err.downcast_ref::<io::Error>() {
                (*err).to_string()
            } else {
                String::from("(unknown)")
            };

            wui_abort!(
                "\
                    Panic: {}\r\n\
                    \r\n\
                    wnd: 0x{:p}\r\n\
                    message: 0x{:x}\r\n\
                    w_param: 0x{:x}\r\n\
                    l_param: 0x{:x}\
                ",
                msg, wnd, message, w_param, l_param)
        }
    }
}
