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
