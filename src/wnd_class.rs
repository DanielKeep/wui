use std::io;
use std::mem;
use std::ptr;
use conv::prelude::*;
use user32;
use winapi::*;
use wio::wide::ToWide;
use ::last_error;
use ::traits::{AsId, IdThunk, IntoRaw};
use ::util::{
    WCString, TryDrop,
    Maybe, Unset, Set,
};

pub type WndProcRef = unsafe extern "system" fn(wnd: HWND, message: UINT, w_param: WPARAM, l_param: LPARAM) -> LRESULT;

pub struct WndClass(ATOM, HINSTANCE);

impl WndClass {
    pub fn new() -> WndClassBuilder {
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
    WndProc = Unset,
    ClsExtra = Unset,
    WndExtra = Unset,
    Instance = Unset,
    Cursor = Unset,
    ClassName = Unset,
    Background = Unset,
> {
    wnd_proc: WndProc,
    cls_extra: ClsExtra,
    wnd_extra: WndExtra,
    instance: Instance,
    cursor: Cursor,
    background: Background,
    class_name: ClassName,
}

impl WndClassBuilder {
    fn new() -> Self {
        WndClassBuilder {
            wnd_proc: Unset,
            cls_extra: Unset,
            wnd_extra: Unset,
            instance: Unset,
            cursor: Unset,
            background: Unset,
            class_name: Unset,
        }
    }
}

impl<T0, T1, T2, T3, T4, T5> WndClassBuilder<Unset, T3, T4, T0, T1, T2, T5> {
    pub fn wnd_proc(self, wnd_proc: WndProcRef) -> WndClassBuilder<WndProcRef, T3, T4, T0, T1, T2, T5> {
        WndClassBuilder {
            wnd_proc: wnd_proc,
            cls_extra: self.cls_extra,
            wnd_extra: self.wnd_extra,
            instance: self.instance,
            cursor: self.cursor,
            class_name: self.class_name,
            background: self.background,
        }
    }
}

impl<T0, T1, T2, T3, T4, T5> WndClassBuilder<T0, T3, T4, Unset, T1, T2, T5> {
    pub fn instance(self, instance: HINSTANCE) -> WndClassBuilder<T0, T3, T4, HINSTANCE, T1, T2, T5> {
        WndClassBuilder {
            wnd_proc: self.wnd_proc,
            cls_extra: self.cls_extra,
            wnd_extra: self.wnd_extra,
            instance: instance,
            cursor: self.cursor,
            class_name: self.class_name,
            background: self.background,
        }
    }
}

impl<T0, T1, T2, T3, T4, T5> WndClassBuilder<T0, T3, T4, T1, Unset, T2, T5> {
    pub fn cursor<Cursor: IntoRaw<Raw=HCURSOR>>(self, cursor: Cursor) -> WndClassBuilder<T0, T3, T4, T1, Set<HCURSOR>, T2, T5> {
        WndClassBuilder {
            wnd_proc: self.wnd_proc,
            cls_extra: self.cls_extra,
            wnd_extra: self.wnd_extra,
            instance: self.instance,
            cursor: Set(cursor.into_raw()),
            class_name: self.class_name,
            background: self.background,
        }
    }
}

impl<T0, T1, T2, T3, T4, T5> WndClassBuilder<T0, T3, T4, T1, T2, Unset, T5> {
    pub fn class_name(self, class_name: &str) -> WndClassBuilder<T0, T3, T4, T1, T2, &str, T5> {
        WndClassBuilder {
            wnd_proc: self.wnd_proc,
            cls_extra: self.cls_extra,
            wnd_extra: self.wnd_extra,
            instance: self.instance,
            cursor: self.cursor,
            class_name: class_name,
            background: self.background,
        }
    }
}

impl<T0, T1, T2, T3, T4, T5> WndClassBuilder<T0, T3, T4, T1, T2, T5, Unset> {
    pub fn background<Background: IntoRaw<Raw=HBRUSH>>(self, background: Background) -> WndClassBuilder<T0, T3, T4, T1, T2, T5, Set<HBRUSH>> {
        WndClassBuilder {
            wnd_proc: self.wnd_proc,
            cls_extra: self.cls_extra,
            wnd_extra: self.wnd_extra,
            instance: self.instance,
            cursor: self.cursor,
            class_name: self.class_name,
            background: Set(background.into_raw()),
        }
    }
}

impl<'a, ClsExtra, WndExtra, Cursor, Background>
WndClassBuilder<WndProcRef, ClsExtra, WndExtra, HINSTANCE, Cursor, &'a str, Background>
where
    ClsExtra: Maybe<usize>,
    WndExtra: Maybe<usize>,
    Cursor: Maybe<HCURSOR>,
    Background: Maybe<HBRUSH>,
{
    pub fn register(self) -> io::Result<WndClass> {
        unsafe {
            let wnd_proc = self.wnd_proc;
            let cls_extra = try!(usize_2_int(self.cls_extra.into_option().unwrap_or(0)));
            let wnd_extra = try!(usize_2_int(self.wnd_extra.into_option().unwrap_or(0)));
            let instance = self.instance;
            let cursor = self.cursor.into_option().unwrap_or(ptr::null_mut());
            let class_name = self.class_name.to_wide_null();
            let class_name = class_name.as_ptr();
            let background = self.background.into_option().unwrap_or(ptr::null_mut());

            let wnd_class = WNDCLASSEXW {
                cbSize: mem::size_of::<WNDCLASSEXW>().value_into().unwrap_ok(),
                style: 0,
                lpfnWndProc: Some(wnd_proc),
                cbClsExtra: cls_extra,
                cbWndExtra: wnd_extra,
                hInstance: instance,
                hIcon: ptr::null_mut(),
                hCursor: cursor,
                hbrBackground: background,
                lpszMenuName: ptr::null_mut(),
                lpszClassName: class_name,
                hIconSm: ptr::null_mut(),
            };

            WndClass::register_raw(&wnd_class)
        }
    }
}

fn usize_2_int(v: usize) -> io::Result<i32> {
    v.value_as::<i32>()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
}
