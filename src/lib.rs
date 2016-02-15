#[macro_use] extern crate conv;
#[macro_use] extern crate custom_derive;
extern crate gdi32;
extern crate kernel32;
extern crate user32;
extern crate winapi;
extern crate wio;

#[macro_use] mod macros;
#[macro_use] mod bitflags;

pub mod ext {
    pub use msg::MsgExt;
}

#[doc(inline)] pub use brush::*;
#[doc(inline)] pub use button::*;
#[doc(inline)] pub use cursor::*;
#[doc(inline)] pub use dc::*;
#[doc(inline)] pub use debug::*;
#[doc(inline)] pub use dialog::*;
#[doc(inline)] pub use dll::*;
#[doc(inline)] pub use icon::*;
#[doc(inline)] pub use msg::*;
#[doc(inline)] pub use paint::*;
#[doc(inline)] pub use text::*;
#[doc(inline)] pub use wnd::*;
#[doc(inline)] pub use wnd_class::*;
#[doc(inline)] pub use wnd_proc::*;

mod brush;
mod button;
mod cursor;
mod dc;
mod debug;
mod dialog;
mod dll;
mod icon;
mod msg;
mod paint;
mod text;
mod traits;
mod util;
mod wnd;
mod wnd_class;
mod wnd_proc;

#[cfg(not(debug_assertions))]
pub fn wui_abort(msg: &str, title: Option<&str>) -> ! {
    use ::std::io::Write;
    let _ = writeln!(::std::io::stderr(), "{}", msg);
    let _ = message_box(None, msg, title, Some(message_box_type::IconError));
    std::process::exit(1);
}

#[cfg(debug_assertions)]
pub fn wui_abort(msg: &str, title: Option<&str>) -> ! {
    unsafe {
        use ::std::io::Write;
        let _ = writeln!(::std::io::stderr(), "{}", msg);
        let _ = title;
        kernel32::DebugBreak();
        std::process::exit(1);
    }
}

fn last_error<T>() -> std::io::Result<T> {
    Err(std::io::Error::last_os_error())
}

fn other_error<T>(msg: &str) -> std::io::Result<T> {
    Err(std::io::Error::new(std::io::ErrorKind::Other, msg))
}
