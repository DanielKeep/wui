#[macro_use] extern crate conv;
#[macro_use] extern crate custom_derive;
extern crate kernel32;
extern crate user32;
extern crate winapi;
extern crate wio;

#[macro_use] mod macros;
#[macro_use] mod bitflags;

pub mod ext {
    pub use msg::MsgExt;
}

#[doc(inline)] pub use cursor::*;
#[doc(inline)] pub use dialog::*;
#[doc(inline)] pub use dll::*;
#[doc(inline)] pub use msg::*;
#[doc(inline)] pub use wnd::*;
#[doc(inline)] pub use wnd_class::*;
#[doc(inline)] pub use wnd_proc::*;

mod cursor;
mod dialog;
mod dll;
mod msg;
mod traits;
mod util;
mod wnd;
mod wnd_class;
mod wnd_proc;

fn last_error<T>() -> std::io::Result<T> {
    Err(std::io::Error::last_os_error())
}

fn other_error<T>(msg: &str) -> std::io::Result<T> {
    Err(std::io::Error::new(std::io::ErrorKind::Other, msg))
}
