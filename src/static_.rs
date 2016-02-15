use std::ops::BitOr;
use winapi::*;
use super::wnd::WndStyle;

const SS_LEFT: DWORD = 0x00000000;
const SS_CENTER: DWORD = 0x00000001;
const SS_RIGHT: DWORD = 0x00000002;
const SS_ICON: DWORD = 0x00000003;
const SS_BLACKRECT: DWORD = 0x00000004;
const SS_GRAYRECT: DWORD = 0x00000005;
const SS_WHITERECT: DWORD = 0x00000006;
const SS_BLACKFRAME: DWORD = 0x00000007;
const SS_GRAYFRAME: DWORD = 0x00000008;
const SS_WHITEFRAME: DWORD = 0x00000009;
const SS_USERITEM: DWORD = 0x0000000A;
const SS_SIMPLE: DWORD = 0x0000000B;
const SS_LEFTNOWORDWRAP: DWORD = 0x0000000C;
const SS_OWNERDRAW: DWORD = 0x0000000D;
const SS_BITMAP: DWORD = 0x0000000E;
const SS_ENHMETAFILE: DWORD = 0x0000000F;
const SS_ETCHEDHORZ: DWORD = 0x00000010;
const SS_ETCHEDVERT: DWORD = 0x00000011;
const SS_ETCHEDFRAME: DWORD = 0x00000012;
const SS_TYPEMASK: DWORD = 0x0000001F;
const SS_REALSIZECONTROL: DWORD = 0x00000040;
const SS_NOPREFIX: DWORD = 0x00000080;
const SS_NOTIFY: DWORD = 0x00000100;
const SS_CENTERIMAGE: DWORD = 0x00000200;
const SS_RIGHTJUST: DWORD = 0x00000400;
const SS_REALSIZEIMAGE: DWORD = 0x00000800;
const SS_SUNKEN: DWORD = 0x00001000;
const SS_EDITCONTROL: DWORD = 0x00002000;
const SS_ENDELLIPSIS: DWORD = 0x00004000;
const SS_PATHELLIPSIS: DWORD = 0x00008000;
const SS_WORDELLIPSIS: DWORD = 0x0000C000;
const SS_ELLIPSISMASK: DWORD = 0x0000C000;

bitflags! {
    flags StaticStyle, static_style: DWORD {
        const Left = super::SS_LEFT,
        const Center = super::SS_CENTER,
        const Right = super::SS_RIGHT,
        const Icon = super::SS_ICON,
        const BlackRect = super::SS_BLACKRECT,
        const GrayRect = super::SS_GRAYRECT,
        const WhiteRect = super::SS_WHITERECT,
        const BlackFrame = super::SS_BLACKFRAME,
        const GrayFrame = super::SS_GRAYFRAME,
        const WhiteFrame = super::SS_WHITEFRAME,
        const UserItem = super::SS_USERITEM,
        const Simple = super::SS_SIMPLE,
        const LeftNoWordWrap = super::SS_LEFTNOWORDWRAP,
        const OwnerDraw = super::SS_OWNERDRAW,
        const Bitmap = super::SS_BITMAP,
        const EnhMetafile = super::SS_ENHMETAFILE,
        const EtchedHorz = super::SS_ETCHEDHORZ,
        const EtchedVert = super::SS_ETCHEDVERT,
        const EtchedFrame = super::SS_ETCHEDFRAME,
        const TypeMask = super::SS_TYPEMASK,
        const RealSizeControl = super::SS_REALSIZECONTROL,
        const NoPrefix = super::SS_NOPREFIX,
        const Notify = super::SS_NOTIFY,
        const CenterImage = super::SS_CENTERIMAGE,
        const RightJust = super::SS_RIGHTJUST,
        const RealSizeImage = super::SS_REALSIZEIMAGE,
        const Sunken = super::SS_SUNKEN,
        const EditControl = super::SS_EDITCONTROL,
        const EndEllipsis = super::SS_ENDELLIPSIS,
        const PathEllipsis = super::SS_PATHELLIPSIS,
        const WordEllipsis = super::SS_WORDELLIPSIS,
        const EllipsisMask = super::SS_ELLIPSISMASK,
    }
}

impl BitOr<StaticStyle> for WndStyle {
    type Output = WndStyle;

    fn bitor(self, other: StaticStyle) -> WndStyle {
        self | WndStyle::from_bits(other.bits)
    }
}
