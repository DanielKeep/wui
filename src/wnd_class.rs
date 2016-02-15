use std::io;
use std::mem;
use std::ptr;
use conv::prelude::*;
use user32;
use winapi::*;
use ::last_error;
use ::traits::{AsId, IdThunk, IntoRaw};
use ::util::{WCString, TryDrop};

pub type WndProcRef = unsafe extern "system" fn(wnd: HWND, message: UINT, w_param: WPARAM, l_param: LPARAM) -> LRESULT;

bitflags! {
    flags WndClassStyle, wnd_class_style: DWORD {
        const Ime = ::winapi::CS_IME,
        const OwnDc = ::winapi::CS_OWNDC,
        const ClassDc = ::winapi::CS_CLASSDC,
        const DblClks = ::winapi::CS_DBLCLKS,
        const HRedraw = ::winapi::CS_HREDRAW,
        const NoClose = ::winapi::CS_NOCLOSE,
        const VRedraw = ::winapi::CS_VREDRAW,
        const ParentDc = ::winapi::CS_PARENTDC,
        const SaveBits = ::winapi::CS_SAVEBITS,
        const DropShadow = ::winapi::CS_DROPSHADOW,
        const GlobalClass = ::winapi::CS_GLOBALCLASS,
        const ByteAlignClient = ::winapi::CS_BYTEALIGNCLIENT,
        const ByteAlignWindow = ::winapi::CS_BYTEALIGNWINDOW,
    }
}

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

pub struct WndClassBuilder {
    style: Option<WndClassStyle>,
    wnd_proc: Option<WndProcRef>,
    cls_extra: Option<usize>,
    wnd_extra: Option<usize>,
    instance: Option<HINSTANCE>,
    icon: Option<HICON>,
    cursor: Option<HCURSOR>,
    background: Option<HBRUSH>,
    class_name: Option<WCString>,
}

impl WndClassBuilder {
    fn new() -> Self {
        WndClassBuilder {
            style: None,
            wnd_proc: None,
            cls_extra: None,
            wnd_extra: None,
            instance: None,
            icon: None,
            cursor: None,
            background: None,
            class_name: None,
        }
    }

    pub fn style(self, style: WndClassStyle) -> WndClassBuilder {
        WndClassBuilder {
            style: Some(style),
            ..self
        }
    }

    pub fn wnd_proc(self, wnd_proc: WndProcRef) -> WndClassBuilder {
        WndClassBuilder {
            wnd_proc: Some(wnd_proc),
            ..self
        }
    }

    pub fn instance(self, instance: HINSTANCE) -> WndClassBuilder {
        WndClassBuilder {
            instance: Some(instance),
            ..self
        }
    }

    pub fn icon<Cursor: IntoRaw<Raw=HICON>>(self, icon: Cursor) -> WndClassBuilder {
        WndClassBuilder {
            icon: Some(icon.into_raw()),
            ..self
        }
    }

    pub fn cursor<Cursor: IntoRaw<Raw=HCURSOR>>(self, cursor: Cursor) -> WndClassBuilder {
        WndClassBuilder {
            cursor: Some(cursor.into_raw()),
            ..self
        }
    }

    pub fn class_name(self, class_name: &str) -> WndClassBuilder {
        WndClassBuilder {
            class_name: Some(class_name.into()),
            ..self
        }
    }

    pub fn background<Background: IntoRaw<Raw=HBRUSH>>(self, background: Background) -> WndClassBuilder {
        WndClassBuilder {
            background: Some(background.into_raw()),
            ..self
        }
    }

    pub fn register(self) -> io::Result<WndClass> {
        unsafe {
            let style = self.style.map(|v| v.bits).unwrap_or(0);
            let wnd_proc = self.wnd_proc.expect("missing wnd_class");
            let cls_extra = try!(usize_2_int(self.cls_extra.unwrap_or(0)));
            let wnd_extra = try!(usize_2_int(self.wnd_extra.unwrap_or(0)));
            let instance = self.instance.expect("missing instance");
            let icon = self.icon.unwrap_or(ptr::null_mut());
            let cursor = self.cursor.unwrap_or(ptr::null_mut());
            let class_name = self.class_name.expect("missing class_name");
            let class_name = class_name.as_ptr();
            let background = self.background.unwrap_or(ptr::null_mut());

            let wnd_class = WNDCLASSEXW {
                cbSize: mem::size_of::<WNDCLASSEXW>().value_into().unwrap(),
                style: style,
                lpfnWndProc: Some(wnd_proc),
                cbClsExtra: cls_extra,
                cbWndExtra: wnd_extra,
                hInstance: instance,
                hIcon: icon,
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
