use std::io;
use std::ptr;
use user32;
use winapi::*;
use wio::wide::ToWide;
use conv::TryFrom;
use ::{last_error, other_error};

bitflags! {
    flags MessageBoxType, message_box_type: UINT {
        const AbortRetryIgnore = 0x2,
        const CanelTryContinue = 0x6,
        const Help = 0x4000,
        const Ok = 0x0,
        const OkCancel = 0x1,
        const RetryCancel = 0x5,
        const YesNo = 0x4,
        const YesNoCancel = 0x3,

        const IconExclamation = 0x30,
        const IconWarning = 0x30,
        const IconInformation = 0x40,
        const IconAsterisk = 0x40,
        const IconQuestion = 0x20,
        const IconStop = 0x10,
        const IconError = 0x10,
        const IconHand = 0x10,

        const DefButton1 = 0x000,
        const DefButton2 = 0x100,
        const DefButton3 = 0x200,
        const DefButton4 = 0x300,

        const ApplModal = 0x0000,
        const SystemModal = 0x1000,
        const TaskModal = 0x2000,

        const DefaultDesktopOnly = 0x20_000,
        const Right = 0x80_000,
        const RtlReading = 0x100_000,
        const SetForeground = 0x10_000,
        const TopMost = 0x40_000,
        const ServiceNotification = 0x200_000,
    }
}

custom_derive! {
    #[derive(Debug, TryFrom(::std::os::raw::c_int))]
    pub enum MessageBoxResult {
        Abort = 3,
        Cancel = 2,
        Continue = 11,
        Ignore = 5,
        No = 7,
        Ok = 1,
        Retry = 4,
        TryAgain = 10,
        Yes = 6,
    }
}

pub fn message_box(wnd: Option<HWND>, text: &str, caption: Option<&str>, type_: Option<MessageBoxType>) -> io::Result<MessageBoxResult> {
    unsafe {
        let wnd = wnd.unwrap_or(ptr::null_mut());
        let text = text.to_wide_null();
        let text = text.as_ptr();
        let caption = caption.map(|v| v.to_wide_null());
        let caption = caption.as_ref().map(|v| v.as_ptr()).unwrap_or(ptr::null_mut());
        let type_ = type_.map(|v| v.bits).unwrap_or(0);
        match user32::MessageBoxW(wnd, text, caption, type_) {
            0 => last_error(),
            v => MessageBoxResult::try_from(v)
                .or_else(|_| other_error("unexpected result"))
        }
    }
}
