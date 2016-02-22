#[macro_use] extern crate scan_rules;
#[macro_use] extern crate wui;
extern crate winapi;

use std::marker::PhantomData;
use scan_rules::ScanError;
use scan_rules::input::ScanInput;
use scan_rules::scanner::{Word, ScanFromStr};
use scan_rules::scanner::{ScanFromBinary, ScanFromHex, ScanFromOctal};
use winapi::*;
use wui::*;

fn main() {
    println!("send-message {}", env!("CARGO_PKG_VERSION"));
    println!("Type `help` for usage.");

    let mut running = true;
    while running {
        print!("> ");
        readln! {
            ("exit") => {
                running = false;
            },
            ("help") => {
                println!("...");
            },
            ("lookup", let msg: Msg) => {
                println!("{:?}", FormatMsg(msg));
            },
            ("quit") => {
                running = false;
            },
            ("send", let wnd: Int<usize>, let msg: Msg) => {
                let wnd = wnd as HWND;
                println!("Sending {:?} to {:p}...", FormatMsg(msg), wnd);
                match unsafe { send_message(wnd, msg, 0, 0) } {
                    Ok(v) => println!(" -> {:?}", v),
                    Err(err) => println!(" -> error: {}", err),
                }
            },
            (let other: Word, .._) => {
                println!("Unknown command `{}`.  Type `help` for usage.", other);
            }
        };
    }
}

struct Int<Output>(PhantomData<Output>);

impl<'a, Output> ScanFromStr<'a> for Int<Output>
where
    Output: std::fmt::Debug
        + ScanFromBinary<'a>
        + ScanFromHex<'a>
        + ScanFromOctal<'a>
        + ScanFromStr<'a, Output=Output>,
{
    type Output = Output;

    fn scan_from<I: ScanInput<'a>>(s: I) -> Result<(Output, usize), ScanError> {
        use ::std::cmp::min;
        let compare = <<I as ScanInput<'a>>::StrCompare as scan_rules::input::StrCompare>::compare;

        let s_str = s.as_str();
        let split = min(2, s_str.len());
        let (prefix, tail) = s_str.split_at(split);
        let r = if compare(prefix, "0b") || compare(prefix, "0B") {
            Output::scan_from_binary(s.from_subslice(tail))
                .map(|(v, o)| (v, o+split))
        } else if compare(prefix, "0o") || compare(prefix, "0O") {
            Output::scan_from_octal(s.from_subslice(tail))
                .map(|(v, o)| (v, o+split))
        } else if compare(prefix, "0x") || compare(prefix, "0X") {
            Output::scan_from_hex(s.from_subslice(tail))
                .map(|(v, o)| (v, o+split))
        } else {
            Output::scan_from(s)
        };
        r
    }
}

struct Msg;

impl<'a> ScanFromStr<'a> for Msg {
    type Output = UINT;

    fn scan_from<I: ScanInput<'a>>(s: I) -> Result<(Self::Output, usize), ScanError> {
        use scan_rules::input::ScanCursor;
        let r = scan! {
            s.to_cursor();
            // ("0x", let x: Hex<UINT>, ^..tail) => (Msg(x), tail.offset()),
            // (let d: UINT, ^..tail) => (Msg(d), tail.offset()),
            (let v: Int<UINT>, ^..tail) => {
                (v, tail.offset())
            },
            (let n: Word, ^..tail) => {
                let end = tail.offset();
                if let Some(v) = message_num(n) {
                    (v, end)
                } else {
                    let v = register_window_message(n)
                        .expect(&format!("could not register message `{}`",
                            n));
                    (v, end)
                }
            },
        };
        r
    }
}
