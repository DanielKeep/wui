use std::io;
use std::mem;
use std::ptr;
use user32;
use winapi::*;
use ::last_error;
use ::traits::{AsId, IdThunk, AsRaw, IntoRaw};
use ::util::TryDrop;
use ::util::WCString;

enum Shared { Yes, No }

pub struct Cursor(HCURSOR, Shared);

impl Cursor {
    pub fn load<Name>(instance: Option<HINSTANCE>, cursor_name: Name) -> io::Result<Cursor>
    where Name: AsId<CursorId> {
        unsafe {
            let instance = instance.unwrap_or(ptr::null_mut());
            let cursor_name = cursor_name.into_id_thunk();
            let cursor_name = cursor_name.as_id();
            let cursor_name = cursor_name.as_raw();
            match user32::LoadCursorW(instance, cursor_name) {
                v if v.is_null() => last_error(),
                v => Ok(Cursor(v, Shared::Yes))
            }
        }
    }
}

impl AsRaw for Cursor {
    type Raw = HCURSOR;

    fn as_raw(&self) -> Self::Raw {
        self.0
    }
}

impl AsRaw for HCURSOR {
    type Raw = HCURSOR;

    fn as_raw(&self) -> Self::Raw {
        *self
    }
}

impl IntoRaw for Cursor {
    fn into_raw(self) -> Self::Raw {
        let r = self.0;
        mem::forget(self);
        r
    }
}

impl IntoRaw for HCURSOR {
    fn into_raw(self) -> Self::Raw {
        self
    }
}

impl Drop for Cursor {
    fn drop(&mut self) {
        unsafe { self.try_drop_inner().unwrap() }
    }
}

impl TryDrop for Cursor {
    type Err = io::Error;

    unsafe fn try_drop_inner(&mut self) -> Result<(), Self::Err> {
        if let Shared::No = self.1 {
            match user32::DestroyCursor(self.0) {
                0 => last_error(),
                _ => Ok(())
            }
        } else {
            Ok(())
        }
    }
}

pub struct CursorId(LPCWSTR);

impl AsRaw for CursorId {
    type Raw = LPCWSTR;

    fn as_raw(&self) -> Self::Raw {
        self.0
    }
}

impl<'a> AsId<CursorId> for &'a str {
    type IdThunk = WCString;
    fn into_id_thunk(self) -> Self::IdThunk {
        self.into()
    }
}

impl IdThunk<CursorId> for WCString {
    fn as_id(&self) -> CursorId {
        CursorId(self.as_ptr())
    }
}

impl AsId<CursorId> for LPCWSTR {
    type IdThunk = LPCWSTR;
    fn into_id_thunk(self) -> Self::IdThunk {
        self
    }
}

impl IdThunk<CursorId> for LPCWSTR {
    fn as_id(&self) -> CursorId {
        CursorId(*self)
    }
}
