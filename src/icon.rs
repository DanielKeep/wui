use std::io;
use std::mem;
use std::ptr;
use user32;
use winapi::*;
use ::last_error;
use ::traits::{AsId, IdThunk, AsRaw, IntoRaw};
use ::util::{Shared, TryDrop, WCString};

pub struct Icon(HICON, Shared);

impl Icon {
    pub fn load<Name>(instance: Option<HINSTANCE>, icon_name: Name) -> io::Result<Icon>
    where Name: AsId<IconId> {
        unsafe {
            let instance = instance.unwrap_or(ptr::null_mut());
            let icon_name = icon_name.into_id_thunk();
            let icon_name = icon_name.as_id().as_raw();
            match user32::LoadIconW(instance, icon_name) {
                v if v.is_null() => last_error(),
                v => Ok(Icon(v, Shared::Yes))
            }
        }
    }
}

impl AsRaw for Icon {
    type Raw = HICON;
    fn as_raw(&self) -> Self::Raw {
        self.0
    }
}

impl AsRaw for HICON {
    type Raw = Self;
    fn as_raw(&self) -> Self {
        *self
    }
}

impl Drop for Icon {
    fn drop(&mut self) {
        unsafe { self.try_drop_inner().unwrap() }
    }
}

impl IntoRaw for Icon {
    fn into_raw(self) -> Self::Raw {
        let r = self.0;
        mem::forget(self);
        r
    }
}

impl IntoRaw for HICON {
    fn into_raw(self) -> Self {
        self
    }
}

impl TryDrop for Icon {
    type Err = io::Error;

    unsafe fn try_drop_inner(&mut self) -> Result<(), Self::Err> {
        if let Shared::No = self.1 {
            match user32::DestroyIcon(self.0) {
                0 => last_error(),
                _ => Ok(())
            }
        } else {
            Ok(())
        }
    }
}

pub struct IconId(LPCWSTR);

impl AsRaw for IconId {
    type Raw = LPCWSTR;

    fn as_raw(&self) -> Self::Raw {
        self.0
    }
}

impl<'a> AsId<IconId> for &'a str {
    type IdThunk = WCString;
    fn into_id_thunk(self) -> Self::IdThunk {
        self.into()
    }
}

impl IdThunk<IconId> for WCString {
    fn as_id(&self) -> IconId {
        IconId(self.as_ptr())
    }
}

impl AsId<IconId> for LPCWSTR {
    type IdThunk = LPCWSTR;
    fn into_id_thunk(self) -> Self::IdThunk {
        self
    }
}

impl IdThunk<IconId> for LPCWSTR {
    fn as_id(&self) -> IconId {
        IconId(*self)
    }
}
