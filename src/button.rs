use std::ops::BitOr;
use winapi::*;
use ::traits::AsRaw;
use super::wnd::{WndBuilder, WndStyle};

bitflags! {
    flags ButtonStyle, button_style: DWORD {
        const Top = ::winapi::BS_TOP,
        const Flat = ::winapi::BS_FLAT,
        const Icon = ::winapi::BS_ICON,
        const Left = ::winapi::BS_LEFT,
        const Text = ::winapi::BS_TEXT,
        const Right = ::winapi::BS_RIGHT,
        const ThreeState = ::winapi::BS_3STATE,
        const Bitmap = ::winapi::BS_BITMAP,
        const Bottom = ::winapi::BS_BOTTOM,
        const Center = ::winapi::BS_CENTER,
        const Notify = ::winapi::BS_NOTIFY,
        const PushBox = ::winapi::BS_PUSHBOX,
        const VCenter = ::winapi::BS_VCENTER,
        const CheckBox = ::winapi::BS_CHECKBOX,
        const GroupBox = ::winapi::BS_GROUPBOX,
        const LeftText = ::winapi::BS_LEFTTEXT,
        const PushLike = ::winapi::BS_PUSHLIKE,
        const TypeMask = ::winapi::BS_TYPEMASK,
        const MultiLine = ::winapi::BS_MULTILINE,
        const OwnerDraw = ::winapi::BS_OWNERDRAW,
        const AutoThreeState = ::winapi::BS_AUTO3STATE,
        const PushButton = ::winapi::BS_PUSHBUTTON,
        const UserButton = ::winapi::BS_USERBUTTON,
        const RadioButton = ::winapi::BS_RADIOBUTTON,
        const RightButton = ::winapi::BS_RIGHTBUTTON,
        const AutoCheckBox = ::winapi::BS_AUTOCHECKBOX,
        const DefPushButton = ::winapi::BS_DEFPUSHBUTTON,
        const AutoRadioButton = ::winapi::BS_AUTORADIOBUTTON,
    }
}

impl BitOr<ButtonStyle> for WndStyle {
    type Output = WndStyle;

    fn bitor(self, other: ButtonStyle) -> WndStyle {
        self | WndStyle::from_bits(other.bits)
    }
}

pub enum Button {}

impl Button {
    pub fn new<'a, Wnd>(wnd_parent: Wnd, id: u16) -> WndBuilder<'a>
    where Wnd: AsRaw<Raw=HWND> {
        super::wnd::Wnd::new()
            .class_name("BUTTON")
            .wnd_parent(&wnd_parent)
            .menu(id as usize as HMENU)
    }
}
