use std::io;
use std::ptr;
use kernel32;
use user32;
use winapi::*;
use ::last_error;
use ::traits::{AsId, IdThunk, AsRaw};
use ::util::{TryDrop, WCString};
use super::wnd_class::WndClassId;

custom_derive! {
    #[derive(Debug, IntoRepr(INT), TryFrom(INT))]
    #[repr(i32)]
    pub enum Show {
        /*  0 */ Hide = SW_HIDE,
        /*  1 */ ShowNormal = SW_SHOWNORMAL,
        /*  2 */ ShowMinimized = SW_SHOWMINIMIZED,
        /*  3 */ Maximize = SW_MAXIMIZE,
        /*  3 */ // ShowMaximized = SW_SHOWMAXIMIZED,
        /*  4 */ ShowNoActivate = SW_SHOWNOACTIVATE,
        /*  5 */ Show = SW_SHOW,
        /*  6 */ Minimize = SW_MINIMIZE,
        /*  7 */ ShowMinNoActive = SW_SHOWMINNOACTIVE,
        /*  8 */ ShowNA = SW_SHOWNA,
        /*  9 */ Restore = SW_RESTORE,
        /* 10 */ ShowDefault = SW_SHOWDEFAULT,
        /* 11 */ ForceMinimize = SW_FORCEMINIMIZE,
    }
}

boolish! {
    #[derive(Debug)]
    pub boolish WasVisible { Yes = true, No = false }
}

bitflags! {
    loose flags WndStyle, wnd_style: DWORD {
        const Border = ::winapi::WS_BORDER,
        const Caption = ::winapi::WS_CAPTION,
        const Child = ::winapi::WS_CHILD,
        const ChildWindow = ::winapi::WS_CHILDWINDOW,
        const ClipChildren = ::winapi::WS_CLIPCHILDREN,
        const ClipSiblings = ::winapi::WS_CLIPSIBLINGS,
        const Disabled = ::winapi::WS_DISABLED,
        const Dlgframe = ::winapi::WS_DLGFRAME,
        const Group = ::winapi::WS_GROUP,
        const HScroll = ::winapi::WS_HSCROLL,
        const Iconic = ::winapi::WS_ICONIC,
        const Maximize = ::winapi::WS_MAXIMIZE,
        const MaximizeBox = ::winapi::WS_MAXIMIZEBOX,
        const Minimize = ::winapi::WS_MINIMIZE,
        const MinimizeBox = ::winapi::WS_MINIMIZEBOX,
        const Overlapped = ::winapi::WS_OVERLAPPED,
        const OverlappedWindow = ::winapi::WS_OVERLAPPEDWINDOW,
        const Popup = ::winapi::WS_POPUP,
        const PopupWindow = ::winapi::WS_POPUPWINDOW,
        const SizeBox = ::winapi::WS_SIZEBOX,
        const SysMenu = ::winapi::WS_SYSMENU,
        const TabStop = ::winapi::WS_TABSTOP,
        const ThickFrame = ::winapi::WS_THICKFRAME,
        const Tiled = ::winapi::WS_TILED,
        const TiledWindow = ::winapi::WS_TILEDWINDOW,
        const Visible = ::winapi::WS_VISIBLE,
        const VScroll = ::winapi::WS_VSCROLL,
    }
}

pub struct Wnd(HWND);

impl Wnd {
    pub fn new<'a>() -> WndBuilder<'a> {
        WndBuilder::new()
    }

    pub unsafe fn create_raw(
        ex_style: DWORD, class_name: LPCWSTR, window_name: LPCWSTR,
        style: DWORD, x: INT, y: INT, width: INT, height: INT,
        wnd_parent: HWND, menu: HMENU, instance: HINSTANCE, param: LPVOID
    ) -> io::Result<Wnd> {
        match user32::CreateWindowExW(
            ex_style, class_name, window_name,
            style, x, y, width, height,
            wnd_parent, menu, instance, param
        ) {
            v if v.is_null() => last_error(),
            v => Ok(Wnd(v))
        }
    }

    pub fn show(&self, cmd_show: Show) -> WasVisible {
        unsafe {
            match user32::ShowWindow(self.0, cmd_show.into_repr()) {
                0 => WasVisible::No,
                _ => WasVisible::Yes,
            }
        }
    }

    pub fn update(&self) -> io::Result<()> {
        unsafe {
            match user32::UpdateWindow(self.0) {
                0 => last_error(),
                _ => Ok(())
            }
        }
    }
}

impl AsRaw for Wnd {
    type Raw = HWND;

    fn as_raw(&self) -> Self::Raw {
        self.0
    }
}

impl AsRaw for HWND {
    type Raw = Self;

    fn as_raw(&self) -> Self {
        *self
    }
}

impl Drop for Wnd {
    fn drop(&mut self) {
        unsafe { self.try_drop_inner().unwrap() }
    }
}

impl TryDrop for Wnd {
    type Err = io::Error;

    unsafe fn try_drop_inner(&mut self) -> Result<(), Self::Err> {
        match user32::DestroyWindow(self.0) {
            0 => last_error(),
            _ => Ok(())
        }
    }
}

pub struct WndBuilder<'a> {
    class_name: Option<Box<IdThunk<WndClassId> + 'a>>,
    window_name: Option<WCString>,
    style: Option<WndStyle>,
    x: Option<INT>,
    y: Option<INT>,
    width: Option<INT>,
    height: Option<INT>,
    wnd_parent: Option<HWND>,
    menu: Option<HMENU>,
    param: Option<LPVOID>,
}

impl<'a> WndBuilder<'a> {
    fn new() -> Self {
        WndBuilder {
            class_name: None,
            window_name: None,
            style: None,
            x: None,
            y: None,
            width: None,
            height: None,
            wnd_parent: None,
            menu: None,
            param: None,
        }
    }

    pub fn class_name<T: 'a + AsId<WndClassId>>(self, value: T) -> Self {
        WndBuilder {
            class_name: Some(Box::new(value.into_id_thunk())),
            ..self
        }
    }

    pub fn window_name(self, value: &str) -> Self {
        WndBuilder {
            window_name: Some(value.into()),
            ..self
        }
    }

    pub fn style(self, value: WndStyle) -> Self {
        WndBuilder {
            style: Some(value),
            ..self
        }
    }

    pub fn x(self, value: INT) -> Self {
        WndBuilder {
            x: Some(value),
            ..self
        }
    }

    pub fn y(self, value: INT) -> Self {
        WndBuilder {
            y: Some(value),
            ..self
        }
    }

    pub fn width(self, value: INT) -> Self {
        WndBuilder {
            width: Some(value),
            ..self
        }
    }

    pub fn height(self, value: INT) -> Self {
        WndBuilder {
            height: Some(value),
            ..self
        }
    }

    pub fn wnd_parent<Wnd: AsRaw<Raw=HWND>>(self, value: Wnd) -> Self {
        WndBuilder {
            wnd_parent: Some(value.as_raw()),
            ..self
        }
    }

    pub fn menu<Menu: AsRaw<Raw=HMENU>>(self, value: Menu) -> Self {
        WndBuilder {
            menu: Some(value.as_raw()),
            ..self
        }
    }

    pub fn button_id(self, value: u16) -> Self {
        WndBuilder {
            menu: Some(value as usize as HMENU),
            ..self
        }
    }

    pub fn param<T>(self, param: *mut T) -> Self {
        WndBuilder {
            param: Some(param as LPVOID),
            ..self
        }
    }

    pub fn create(self) -> io::Result<Wnd> {
        unsafe {
            let ex_style = 0;
            let class_name = self.class_name.expect("missing class_name");
            let (class_name, instance) = class_name.as_id().unpack();
            let window_name = self.window_name.expect("missing window_name");
            let window_name = window_name.as_ptr();
            let style = self.style.expect("missing style").bits;
            let x = self.x.unwrap_or(CW_USEDEFAULT);
            let y = self.y.unwrap_or(CW_USEDEFAULT);
            let width = self.width.unwrap_or(CW_USEDEFAULT);
            let height = self.height.unwrap_or(CW_USEDEFAULT);
            let wnd_parent = self.wnd_parent.unwrap_or(ptr::null_mut());
            let menu = self.menu.unwrap_or(ptr::null_mut());
            let param = self.param.unwrap_or(ptr::null_mut());
            Wnd::create_raw(ex_style, class_name, window_name,
                style, x, y, width, height,
                wnd_parent, menu, instance, param)
        }
    }
}

pub unsafe fn get_window_long_ptr<W, T>(wnd: W, index: i32) -> io::Result<*const T>
where W: AsRaw<Raw=HWND>
{
    #[cfg(target_pointer_width="32")]
    use ::user32::GetWindowLongW as GetWindowLongPtr;

    #[cfg(target_pointer_width="64")]
    use ::user32::GetWindowLongPtrW;

    // Clear so that we can distinguish from "success, and the value was zero" and "failure".
    kernel32::SetLastError(0);

    let wnd = wnd.as_raw();

    match GetWindowLongPtr(wnd, index) {
        0 if kernel32::GetLastError() == 0 => Ok(0usize as *const T),
        0 => last_error(),
        v => Ok(v as *const T)
    }
}

pub unsafe fn set_font<W, F>(wnd: W, font: F, redraw: bool)
where
    W: AsRaw<Raw=HWND>,
    F: AsRaw<Raw=HFONT>,
{
    let wnd = wnd.as_raw();
    let font = font.as_raw() as WPARAM;
    let redraw = redraw as LPARAM;
    user32::SendMessageW(wnd, WM_SETFONT, font, redraw);
}

pub unsafe fn set_window_long_ptr<W, T>(wnd: W, index: i32, new_long: *const T)
-> io::Result<*const T>
where W: AsRaw<Raw=HWND>
{
    #[cfg(target_pointer_width="32")]
    use ::user32::SetWindowLongW as SetWindowLongPtr;

    #[cfg(target_pointer_width="64")]
    use ::user32::SetWindowLongPtrW;

    // Clear so that we can distinguish from "success, and the last value was zero" and "failure".
    kernel32::SetLastError(0);

    let wnd = wnd.as_raw();
    let new_long = new_long as LONG_PTR;

    match SetWindowLongPtr(wnd, index, new_long) {
        0 if kernel32::GetLastError() == 0 => Ok(0usize as *const T),
        0 => last_error(),
        v => Ok(v as *const T)
    }
}
