#[macro_export]
macro_rules! boolish {
    (
        $(#[$($attrs:tt)*])*
        pub boolish $name:ident {
            $yes:ident = true,
            $no:ident = false $(,)*
        }
    ) => {
        boolish! {
            @as_items
            $(#[$($attrs)*])*
            pub enum $name {
                $yes,
                $no
            }

            impl From<bool> for $name {
                fn from(v: bool) -> Self {
                    match v {
                        true => $name::$yes,
                        false => $name::$no,
                    }
                }
            }

            impl From<$name> for bool {
                fn from(v: $name) -> Self {
                    match v {
                        $name::$yes => true,
                        $name::$no => false,
                    }
                }
            }
        }
    };

    (@as_items $($i:item)*) => { $($i)* };
}

#[macro_export]
macro_rules! wui_abort {
    ($e:expr, $($args:tt)*) => {
        match format!($e, $($args)*) {
            msg => $crate::wui_abort(&msg, None)
        }
    };
}

#[macro_export]
macro_rules! wui_no_panic {
    ($($body:tt)*) => {
        match ::std::panic::recover(move || wui_util__!(@as_expr {$($body)*})) {
            Ok(res) => res,
            Err(err) => wui_abort!("Panic: {}", err)
        }
    };
}

#[macro_export]
macro_rules! wui_ok_or_warn {
    ($($body:tt)*) => {
        match (|| -> ::std::io::Result<()> { wui_util__!(@as_expr { $($body)* }) })() {
            Ok(()) => (),
            Err(err) => {
                use ::std::io::Write;
                let _ = write!(::std::io::stderr(), "Warning: ignored error: {}", err);
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! wui_util__ {
    (@as_expr $e:expr) => { $e };
}

macro_rules! io_err {
    ($arg:expr) => {
        Err(::std::io::Error::new(::std::io::ErrorKind::Other, $arg))
    };

    ($arg:expr, $($tail:tt)*) => {
        Err(::std::io::Error::new(::std::io::ErrorKind::Other,
            format!($arg, $($tail)*)))
    };
}

macro_rules! IntoRepr {
    (
        ($repr_ty:ty) pub enum $name:ident $($_tail:tt)*
    ) => {
        impl $name {
            pub fn into_repr(self) -> $repr_ty {
                self as $repr_ty
            }
        }
    };
}
