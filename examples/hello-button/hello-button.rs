#![feature(recover)]
#![feature(type_ascription)]

#[macro_use] extern crate wui;
extern crate winapi;

use std::cell::Cell;
use std::io;
use std::mem;
use std::ptr;
use winapi::*;
use wui::*;
use wui::util::TryDrop;

const BTN_HELLO_ID: u16 = 101;

#[derive(Debug)]
pub struct WndExtra {
    font: Cell<HFONT>,
}

impl Default for WndExtra {
    fn default() -> Self {
        WndExtra {
            font: Cell::new(ptr::null_mut()),
        }
    }
}

impl Drop for WndExtra {
    fn drop(&mut self) {
        unsafe { self.try_drop_inner().unwrap() }
    }
}

impl TryDrop for WndExtra {
    type Err = io::Error;

    unsafe fn try_drop_inner(&mut self) -> Result<(), Self::Err> {
        let mut r = Ok(());
        match self.font.get() {
            v if v.is_null() => (),
            v => {
                self.font.set(ptr::null_mut());
                let font = Font::from_raw(v);
                r = r.and(font.try_drop());
            }
        }
        r
    }
}

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
    use wui::static_style as ss;
    use wui::wnd_style as ws;

    let wnd_class = try!(WndClass::new()
        .class_name("Hello")
        .instance(try!(get_module_handle(None)))
        .icon(try!(Icon::load(None, IDI_APPLICATION)))
        .cursor(try!(Cursor::load(None, IDC_ARROW)))
        .background(try!(Brush::get_sys_color(Color::BtnFace)))
        .wnd_proc(wnd_proc)
        .register());

    // Get a decent default font.
    let msg_font = {
        let ncm = try!(system_parameters_info::get_non_client_metrics());
        let font = try!(Font::create(&ncm.lfMessageFont));
        let font_raw = font.as_raw();
        mem::forget(font);
        font_raw
    };

    // Build extra window data.
    let extra = Box::new(WndExtra::default());
    extra.font.set(msg_font);
    let extra_ptr = Box::into_raw(extra);

    let wnd = try!(Wnd::new()
        .class_name(&wnd_class)
        .window_name("Hello")
        .style(ws::OverlappedWindow)
        .width(250+30).height(45+40)
        .param(extra_ptr)
        .create());

    let lbl = try!(Static::new(&wnd)
        .window_name("Click that over there.")
        .style(ws::Child | ws::Visible | ss::CenterImage)
        .x(10).y(10)
        .width(150).height(25)
        .create());
    unsafe { set_font(&lbl, msg_font, false); }

    let btn = try!(Button::new(&wnd, BTN_HELLO_ID)
        .window_name("Hello")
        .style(ws::TabStop | ws::Visible | ws::Child | bs::DefPushButton)
        .x(170).y(10)
        .width(80).height(25)
        .create());
    unsafe { set_font(&btn, msg_font, false); }

    wnd.show(Show::ShowDefault);
    try!(wnd.update());

    let top_level_wnds = [&wnd];

    loop {
        let msg = try!(MSG::get(None, None));
        trace_message(msg.hwnd, msg.message, msg.wParam, msg.lParam);
        match msg {
            MSG { message: WM_QUIT, wParam: code, .. } => {
                ::std::process::exit(code as i32);
            },
            mut msg => {
                let mut processed = false;
                for wnd in &top_level_wnds {
                    if msg.is_dialog_message(wnd) {
                        processed = true;
                        break;
                    }
                }
                if !processed {
                    msg.translate();
                    msg.dispatch();
                }
            }
        }
    }
}

fn try_wnd_proc(wnd: HWND, message: UINT, w_param: WPARAM, l_param: LPARAM) -> io::Result<LRESULT> {
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
        WM_CREATE => {
            unsafe {
                let l_param_ptr = l_param as *mut CREATESTRUCTW;
                if l_param_ptr.is_null() {
                    return other_error("got null *mut CREATESTRUCT in WM_CREATE");
                }
                let l_param = &*l_param_ptr;

                let extra_ptr = l_param.lpCreateParams as *mut WndExtra;
                if extra_ptr.is_null() {
                    return other_error("got null *mut WndExtra in WM_CREATE");
                }
                let extra = &*extra_ptr;

                let font = extra.font.get();
                set_font(wnd, font, false);

                try!(set_window_long_ptr(wnd, GWLP_USERDATA, extra_ptr));
            }

            Ok(0)
        },
        WM_DESTROY => {
            // Drop extra window data.
            wui_ok_or_warn! {
                unsafe {
                    let extra_ptr = try!(set_window_long_ptr(wnd, GWLP_USERDATA, ptr::null::<WndExtra>()));

                    if extra_ptr.is_null () {
                        return other_error("cannot delete WndExtra: window long was zero");
                    }

                    /*
                    This *should* be OK because we just erased the pointer in the window itself.  MSDN doesn't specify whether or not this is atomic, but it *should* be OK.  There's only so un-atomic it can be.
                    */
                    let extra_ptr = extra_ptr as *mut _;
                    let extra = Box::<WndExtra>::from_raw(extra_ptr);
                    try!(extra.try_drop());
                    Ok(())
                }
            }
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
                wnd: {:p}\r\n\
                message: {:?}\r\n\
                w_param: 0x{:x}\r\n\
                l_param: 0x{:x}\
            ",
            err, wnd, FormatMsg(message), w_param, l_param),
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
                    wnd: {:p}\r\n\
                    message: {:?}\r\n\
                    w_param: 0x{:x}\r\n\
                    l_param: 0x{:x}\
                ",
                msg, wnd, FormatMsg(message), w_param, l_param)
        }
    }
}

fn other_error<T>(msg: &str) -> std::io::Result<T> {
    Err(std::io::Error::new(std::io::ErrorKind::Other, msg))
}
