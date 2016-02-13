use std::io;
use std::mem;
use gdi32;
use user32;
use winapi::*;
use ::last_error;
use ::traits::{AsRaw, IntoRaw};
use ::util::TryDrop;

custom_derive! {
    #[derive(Debug, TryFrom(INT), IntoRepr(INT))]
    #[repr(i32)]
    pub enum Color {
        /*  0 */ ScrollBar = COLOR_SCROLLBAR,
        /*  1 */ Background = COLOR_BACKGROUND,
        /*  2 */ ActiveCaption = COLOR_ACTIVECAPTION,
        /*  3 */ InactiveCaption = COLOR_INACTIVECAPTION,
        /*  4 */ Menu = COLOR_MENU,
        /*  5 */ Window = COLOR_WINDOW,
        /*  6 */ WindowFrame = COLOR_WINDOWFRAME,
        /*  7 */ MenuText = COLOR_MENUTEXT,
        /*  8 */ WindowText = COLOR_WINDOWTEXT,
        /*  9 */ CaptionText = COLOR_CAPTIONTEXT,
        /* 10 */ ActiveBorder = COLOR_ACTIVEBORDER,
        /* 11 */ InactiveBorder = COLOR_INACTIVEBORDER,
        /* 12 */ AppWorkspace = COLOR_APPWORKSPACE,
        /* 13 */ Highlight = COLOR_HIGHLIGHT,
        /* 14 */ HighlightText = COLOR_HIGHLIGHTTEXT,
        /* 15 */ BtnFace = COLOR_BTNFACE,
        /* 16 */ BtnShadow = COLOR_BTNSHADOW,
        /* 17 */ GrayText = COLOR_GRAYTEXT,
        /* 18 */ BtnText = COLOR_BTNTEXT,
        /* 19 */ InactiveCaptionText = COLOR_INACTIVECAPTIONTEXT,
        /* 20 */ BtnHighlight = COLOR_BTNHIGHLIGHT,
        /* 21 */ ThreeDDkShadow = COLOR_3DDKSHADOW,
        /* 22 */ ThreeDLight = COLOR_3DLIGHT,
        /* 23 */ InfoText = COLOR_INFOTEXT,
        /* 24 */ InfoBk = COLOR_INFOBK,
        /* 26 */ Hotlight = COLOR_HOTLIGHT,
        /* 27 */ GradientActiveCaption = COLOR_GRADIENTACTIVECAPTION,
        /* 28 */ GradientInactiveCaption = COLOR_GRADIENTINACTIVECAPTION,
        /* 29 */ MenuHilight = COLOR_MENUHILIGHT,
        /* 30 */ MenuBar = COLOR_MENUBAR,

        // /* COLOR_BACKGROUND */ Desktop = COLOR_DESKTOP,
        // /* COLOR_BTNFACE */ ThreeDFace = COLOR_3DFACE,
        // /* COLOR_BTNSHADOW */ ThreeDShadow = COLOR_3DSHADOW,
        // /* COLOR_BTNHIGHLIGHT */ ThreeDHighlight = COLOR_3DHIGHLIGHT,
        // /* COLOR_BTNHIGHLIGHT */ ThreeDHilight = COLOR_3DHILIGHT,
        // /* COLOR_BTNHIGHLIGHT */ BtnHilight = COLOR_BTNHILIGHT,
    }
}

pub struct Brush(HBRUSH);

impl Brush {
    pub fn get_sys_color(color: Color) -> io::Result<Brush> {
        unsafe {
            match user32::GetSysColorBrush(color.into_repr()) {
                v if v.is_null() => last_error(),
                v => Ok(Brush(v))
            }
        }
    }
}

impl AsRaw for Brush {
    type Raw = HBRUSH;

    fn as_raw(&self) -> HBRUSH {
        self.0
    }
}

impl AsRaw for HBRUSH {
    type Raw = Self;
    fn as_raw(&self) -> Self {
        *self
    }
}

impl Drop for Brush {
    fn drop(&mut self) {
        unsafe { self.try_drop_inner().unwrap() }
    }
}

impl IntoRaw for Brush {
    fn into_raw(self) -> HBRUSH {
        let r = self.0;
        mem::forget(self);
        r
    }
}

impl IntoRaw for HBRUSH {
    fn into_raw(self) -> HBRUSH {
        self
    }
}

impl TryDrop for Brush {
    type Err = io::Error;

    unsafe fn try_drop_inner(&mut self) -> Result<(), Self::Err> {
        match gdi32::DeleteObject(self.0 as *mut _) {
            0 => last_error(),
            _ => Ok(())
        }
    }
}
