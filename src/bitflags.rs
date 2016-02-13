// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! A typesafe bitmask flag generator.

/// The `bitflags!` macro generates a `struct` that holds a set of C-style
/// bitmask flags. It is useful for creating typesafe wrappers for C APIs.
///
/// The flags should only be defined for integer types, otherwise unexpected
/// type errors may occur at compile time.
///
/// # Trait implementations
///
/// The `Copy`, `Clone`, `PartialEq`, `Eq`, `PartialOrd`, `Ord` and `Hash`
/// traits automatically derived for the `struct` using the `derive` attribute.
/// Additional traits can be derived by providing an explicit `derive`
/// attribute on `flags`.
///
/// The `FromIterator` trait is implemented for the `struct`, too, calculating
/// the union of the instances of the `struct` iterated over.
///
/// The `Debug` trait is also implemented by displaying the bits value of the
/// internal struct.
///
/// ## Operators
///
/// The following operator traits are implemented for the generated `struct`:
///
/// - `BitOr`: union
/// - `BitAnd`: intersection
/// - `BitXor`: toggle
/// - `Sub`: set difference
/// - `Not`: set complement
///
/// # Methods
///
/// The following methods are defined for the generated `struct`:
///
/// - `empty`: an empty set of flags
/// - `all`: the set of all flags
/// - `bits`: the raw value of the flags currently stored
/// - `from_bits`: convert from underlying bit representation, unless that
///                representation contains bits that do not correspond to a flag
/// - `from_bits_truncate`: convert from underlying bit representation, dropping
///                         any bits that do not correspond to flags
/// - `is_empty`: `true` if no flags are currently stored
/// - `is_all`: `true` if all flags are currently set
/// - `intersects`: `true` if there are flags common to both `self` and `other`
/// - `contains`: `true` all of the flags in `other` are contained within `self`
/// - `insert`: inserts the specified flags in-place
/// - `remove`: removes the specified flags in-place
/// - `toggle`: the specified flags will be inserted if not present, and removed
///             if they are.
#[macro_export]
macro_rules! bitflags {
    ($(#[$attr:meta])* flags $BitFlags:ident, $bfmod:ident: $T:ty {
        $($(#[$Flag_attr:meta])* const $Flag:ident = $value:expr),+
    }) => {
        #[derive(Copy, PartialEq, Eq, Clone, PartialOrd, Ord, Hash)]
        $(#[$attr])*
        pub struct $BitFlags {
            bits: $T,
        }

        pub mod $bfmod {
            $(
                #[allow(non_upper_case_globals)]
                $(#[$Flag_attr])*
                pub const $Flag: super::$BitFlags = super::$BitFlags { bits: $value };
            )+
        }

        impl ::std::fmt::Debug for $BitFlags {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                // This convoluted approach is to handle #[cfg]-based flag
                // omission correctly. Some of the $Flag variants may not be
                // defined in this module so we create an inner module which
                // defines *all* flags to the value of 0. Afterwards when the
                // glob import variants from the outer module, shadowing all
                // defined variants, leaving only the undefined ones with the
                // bit value of 0.
                #[allow(dead_code)]
                #[allow(unused_assignments)]
                mod dummy {
                    // Now we define the "undefined" versions of the flags.
                    // This way, all the names exist, even if some are #[cfg]ed
                    // out.
                    $(
                        #[allow(non_upper_case_globals)]
                        const $Flag: super::$BitFlags = super::$BitFlags { bits: 0 };
                    )+

                    #[inline]
                    pub fn fmt(self_: &super::$BitFlags,
                               f: &mut ::std::fmt::Formatter)
                               -> ::std::fmt::Result {
                        use super::$bfmod::*;
                        let mut first = true;
                        $(
                            // $Flag.bits == 0 means that $Flag doesn't exist
                            if $Flag.bits != 0 && self_.contains($Flag) {
                                if !first {
                                    try!(f.write_str(" | "));
                                }
                                first = false;
                                try!(f.write_str(stringify!($Flag)));
                            }
                        )+
                        ::std::result::Result::Ok(())
                    }
                }
                dummy::fmt(self, f)
            }
        }

        #[allow(dead_code)]
        impl $BitFlags {
            /// Returns an empty set of flags.
            #[inline]
            pub fn empty() -> $BitFlags {
                $BitFlags { bits: 0 }
            }

            /// Returns the set containing all flags.
            #[inline]
            pub fn all() -> $BitFlags {
                // See above `dummy` module for why this approach is taken.
                #[allow(dead_code)]
                mod dummy {
                    $(
                        #[allow(non_upper_case_globals)]
                        const $Flag: super::$BitFlags = super::$BitFlags { bits: 0 };
                    )+

                    #[inline]
                    pub fn all() -> super::$BitFlags {
                        use super::$bfmod::*;
                        super::$BitFlags { bits: $($Flag.bits)|+ }
                    }
                }
                dummy::all()
            }

            /// Returns the raw value of the flags currently stored.
            #[inline]
            pub fn bits(&self) -> $T {
                self.bits
            }

            /// Convert from underlying bit representation, unless that
            /// representation contains bits that do not correspond to a flag.
            #[inline]
            pub fn from_bits(bits: $T) -> ::std::option::Option<$BitFlags> {
                if (bits & !$BitFlags::all().bits()) != 0 {
                    ::std::option::Option::None
                } else {
                    ::std::option::Option::Some($BitFlags { bits: bits })
                }
            }

            /// Convert from underlying bit representation, dropping any bits
            /// that do not correspond to flags.
            #[inline]
            pub fn from_bits_truncate(bits: $T) -> $BitFlags {
                $BitFlags { bits: bits } & $BitFlags::all()
            }

            /// Returns `true` if no flags are currently stored.
            #[inline]
            pub fn is_empty(&self) -> bool {
                *self == $BitFlags::empty()
            }

            /// Returns `true` if all flags are currently set.
            #[inline]
            pub fn is_all(&self) -> bool {
                *self == $BitFlags::all()
            }

            /// Returns `true` if there are flags common to both `self` and `other`.
            #[inline]
            pub fn intersects(&self, other: $BitFlags) -> bool {
                !(*self & other).is_empty()
            }

            /// Returns `true` all of the flags in `other` are contained within `self`.
            #[inline]
            pub fn contains(&self, other: $BitFlags) -> bool {
                (*self & other) == other
            }

            /// Inserts the specified flags in-place.
            #[inline]
            pub fn insert(&mut self, other: $BitFlags) {
                self.bits |= other.bits;
            }

            /// Removes the specified flags in-place.
            #[inline]
            pub fn remove(&mut self, other: $BitFlags) {
                self.bits &= !other.bits;
            }

            /// Toggles the specified flags in-place.
            #[inline]
            pub fn toggle(&mut self, other: $BitFlags) {
                self.bits ^= other.bits;
            }
        }

        impl ::std::ops::BitOr for $BitFlags {
            type Output = $BitFlags;

            /// Returns the union of the two sets of flags.
            #[inline]
            fn bitor(self, other: $BitFlags) -> $BitFlags {
                $BitFlags { bits: self.bits | other.bits }
            }
        }

        impl ::std::ops::BitXor for $BitFlags {
            type Output = $BitFlags;

            /// Returns the left flags, but with all the right flags toggled.
            #[inline]
            fn bitxor(self, other: $BitFlags) -> $BitFlags {
                $BitFlags { bits: self.bits ^ other.bits }
            }
        }

        impl ::std::ops::BitAnd for $BitFlags {
            type Output = $BitFlags;

            /// Returns the intersection between the two sets of flags.
            #[inline]
            fn bitand(self, other: $BitFlags) -> $BitFlags {
                $BitFlags { bits: self.bits & other.bits }
            }
        }

        impl ::std::ops::Sub for $BitFlags {
            type Output = $BitFlags;

            /// Returns the set difference of the two sets of flags.
            #[inline]
            fn sub(self, other: $BitFlags) -> $BitFlags {
                $BitFlags { bits: self.bits & !other.bits }
            }
        }

        impl ::std::ops::Not for $BitFlags {
            type Output = $BitFlags;

            /// Returns the complement of this set of flags.
            #[inline]
            fn not(self) -> $BitFlags {
                $BitFlags { bits: !self.bits } & $BitFlags::all()
            }
        }

        impl ::std::iter::FromIterator<$BitFlags> for $BitFlags {
            fn from_iter<T: ::std::iter::IntoIterator<Item=$BitFlags>>(iterator: T) -> $BitFlags {
                let mut result = Self::empty();
                for item in iterator {
                    result.insert(item)
                }
                result
            }
        }
    };
    ($(#[$attr:meta])* flags $BitFlags:ident, $bfmod:ident: $T:ty {
        $($(#[$Flag_attr:meta])* const $Flag:ident = $value:expr),+,
    }) => {
        bitflags! {
            $(#[$attr])*
            flags $BitFlags, $bfmod: $T {
                $($(#[$Flag_attr])* const $Flag = $value),+
            }
        }
    };
}
