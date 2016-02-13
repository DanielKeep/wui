use std::io;
use std::mem;
use std::ptr;
use conv::prelude::*;
use user32;
use winapi::*;
use wio::wide::ToWide;
use ::last_error;
use ::traits::{AsId, IdThunk, AsRaw};
use ::util::{
    WCString, TryDrop,
    Maybe, Unset, Set,
};
use super::cursor::Cursor;

pub type WndProcRef = unsafe extern "system" fn(wnd: HWND, message: UINT, w_param: WPARAM, l_param: LPARAM) -> LRESULT;

pub struct WndClass(ATOM, HINSTANCE);

impl WndClass {
    pub fn new() -> WndClassBuilder<Unset, Unset, Unset, Unset> {
        WndClassBuilder::new()
    }

    pub unsafe fn register_raw(wnd_class: &WNDCLASSEXW) -> io::Result<Self> {
        match user32::RegisterClassExW(wnd_class) {
            0 => last_error(),
            v => Ok(WndClass(v, wnd_class.hInstance))
        }
    }
}

impl Drop for WndClass {
    fn drop(&mut self) {
        unsafe { self.try_drop_inner().unwrap() }
    }
}

impl TryDrop for WndClass {
    type Err = io::Error;

    unsafe fn try_drop_inner(&mut self) -> Result<(), Self::Err> {
        let name = self.0 as LPCWSTR;
        match user32::UnregisterClassW(name, self.1) {
            0 => last_error(),
            _ => Ok(())
        }
    }
}

impl<'a> AsId<WndClassId> for &'a WndClass {
    type IdThunk = &'a WndClass;
    fn into_id_thunk(self) -> Self::IdThunk {
        self
    }
}

impl<'a> IdThunk<WndClassId> for &'a WndClass {
    fn as_id(&self) -> WndClassId {
        WndClassId(self.0 as LPCWSTR, self.1)
    }
}

pub struct WndClassId(LPCWSTR, HINSTANCE);

impl WndClassId {
    pub fn class_name(&self) -> LPCWSTR {
        self.0
    }

    pub fn instance(&self) -> HINSTANCE {
        self.1
    }

    pub fn unpack(self) -> (LPCWSTR, HINSTANCE) {
        (self.0, self.1)
    }
}

impl AsId<WndClassId> for &'static str {
    type IdThunk = WCString;
    fn into_id_thunk(self) -> Self::IdThunk {
        self.into()
    }
}

impl IdThunk<WndClassId> for WCString {
    fn as_id(&self) -> WndClassId {
        WndClassId(self.as_ptr(), ptr::null_mut())
    }
}

pub struct WndClassBuilder<
    WndProc,
    Instance,
    Cursor,
    ClassName,
> {
    wnd_proc: WndProc,
    instance: Instance,
    cursor: Cursor,
    class_name: ClassName,
}

impl WndClassBuilder<Unset, Unset, Unset, Unset> {
    fn new() -> Self {
        WndClassBuilder {
            wnd_proc: Unset,
            instance: Unset,
            cursor: Unset,
            class_name: Unset,
        }
    }
}

impl<T0, T1, T2> WndClassBuilder<Unset, T0, T1, T2> {
    pub fn wnd_proc(self, wnd_proc: WndProcRef) -> WndClassBuilder<WndProcRef, T0, T1, T2> {
        WndClassBuilder {
            wnd_proc: wnd_proc,
            instance: self.instance,
            cursor: self.cursor,
            class_name: self.class_name,
        }
    }
}

impl<T0, T1, T2> WndClassBuilder<T0, Unset, T1, T2> {
    pub fn instance(self, instance: HINSTANCE) -> WndClassBuilder<T0, HINSTANCE, T1, T2> {
        WndClassBuilder {
            wnd_proc: self.wnd_proc,
            instance: instance,
            cursor: self.cursor,
            class_name: self.class_name,
        }
    }
}

impl<T0, T1, T2> WndClassBuilder<T0, T1, Unset, T2> {
    pub fn cursor(self, cursor: Cursor) -> WndClassBuilder<T0, T1, Set<Cursor>, T2> {
        WndClassBuilder {
            wnd_proc: self.wnd_proc,
            instance: self.instance,
            cursor: Set(cursor),
            class_name: self.class_name,
        }
    }
}

impl<T0, T1, T2> WndClassBuilder<T0, T1, T2, Unset> {
    pub fn class_name(self, class_name: &str) -> WndClassBuilder<T0, T1, T2, &str> {
        WndClassBuilder {
            wnd_proc: self.wnd_proc,
            instance: self.instance,
            cursor: self.cursor,
            class_name: class_name,
        }
    }
}

impl<'a, Cursor_: Maybe<Cursor>> WndClassBuilder<WndProcRef, HINSTANCE, Cursor_, &'a str> {
    pub fn register(self) -> io::Result<WndClass> {
        unsafe {
            let wnd_proc = self.wnd_proc;
            let instance = self.instance;
            let cursor = self.cursor.into_option();
            let cursor = cursor.as_ref().map(|v| v.as_raw()).unwrap_or(ptr::null_mut());
            let class_name = self.class_name.to_wide_null();
            let class_name = class_name.as_ptr();

            let wnd_class = WNDCLASSEXW {
                cbSize: mem::size_of::<WNDCLASSEXW>().value_into().unwrap_ok(),
                style: 0,
                lpfnWndProc: Some(wnd_proc),
                cbClsExtra: 0,
                cbWndExtra: 0,
                hInstance: instance,
                hIcon: ptr::null_mut(),
                hCursor: cursor,
                hbrBackground: ptr::null_mut(),
                lpszMenuName: ptr::null_mut(),
                lpszClassName: class_name,
                hIconSm: ptr::null_mut(),
            };

            WndClass::register_raw(&wnd_class)
        }
    }
}
