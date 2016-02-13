use std::io;
use std::ptr;
use user32;
use winapi::*;
use wio::wide::ToWide;
use ::last_error;
use ::traits::{AsId, IdThunk, AsRaw};
use ::util::TryDrop;
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
    flags WndStyle, wnd_style: DWORD {
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
    pub fn new() -> WndBuilder<(), (), ()> {
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

pub struct WndBuilder<
    ClassName,
    WindowName,
    Style,
> {
    class_name: ClassName,
    window_name: WindowName,
    style: Style,
}

impl WndBuilder<(), (), ()> {
    fn new() -> Self {
        WndBuilder {
            class_name: (),
            window_name: (),
            style: (),
        }
    }
}

impl<T0, T1> WndBuilder<(), T0, T1> {
    pub fn class_name<T>(self, value: T) -> WndBuilder<T, T0, T1>
    where T: AsId<WndClassId> {
        WndBuilder {
            class_name: value,
            window_name: self.window_name,
            style: self.style,
        }
    }
}

impl<T0, T1> WndBuilder<T0, (), T1> {
    pub fn window_name(self, value: &str) -> WndBuilder<T0, &str, T1> {
        WndBuilder {
            class_name: self.class_name,
            window_name: value,
            style: self.style,
        }
    }
}

impl<T0, T1> WndBuilder<T0, T1, ()> {
    pub fn style(self, value: WndStyle) -> WndBuilder<T0, T1, WndStyle> {
        WndBuilder {
            class_name: self.class_name,
            window_name: self.window_name,
            style: value,
        }
    }
}

impl<'a, ClassName> WndBuilder<ClassName, &'a str, WndStyle>
where ClassName: AsId<WndClassId> {
    pub fn create(self) -> io::Result<Wnd> {
        unsafe {
            let ex_style = 0;
            let class_name = self.class_name.into_id_thunk();
            let (class_name, instance) = class_name.as_id().unpack();
            let window_name = self.window_name.to_wide_null();
            let window_name = window_name.as_ptr();
            let style = self.style.bits;
            let x = CW_USEDEFAULT;
            let y = CW_USEDEFAULT;
            let width = CW_USEDEFAULT;
            let height = CW_USEDEFAULT;
            let wnd_parent = ptr::null_mut();
            let menu = ptr::null_mut();
            let param = ptr::null_mut();
            Wnd::create_raw(ex_style, class_name, window_name,
                style, x, y, width, height,
                wnd_parent, menu, instance, param)
        }
    }
}
