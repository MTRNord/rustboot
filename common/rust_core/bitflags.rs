// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![experimental]
#![macro_escape]

//! A typesafe bitmask flag generator.

/// The `bitflags!` macro generates a `struct` that holds a set of C-style
/// bitmask flags. It is useful for creating typesafe wrappers for C APIs.
///
/// The flags should only be defined for integer types, otherwise unexpected
/// type errors may occur at compile time.
///
/// # Example
///
/// ```{.rust}
/// bitflags! {
///     flags Flags: u32 {
///         const FLAG_A       = 0x00000001,
///         const FLAG_B       = 0x00000010,
///         const FLAG_C       = 0x00000100,
///         const FLAG_ABC     = FLAG_A.bits
///                            | FLAG_B.bits
///                            | FLAG_C.bits,
///     }
/// }
///
/// fn main() {
///     let e1 = FLAG_A | FLAG_C;
///     let e2 = FLAG_B | FLAG_C;
///     assert!((e1 | e2) == FLAG_ABC);   // union
///     assert!((e1 & e2) == FLAG_C);     // intersection
///     assert!((e1 - e2) == FLAG_A);     // set difference
///     assert!(!e2 == FLAG_A);           // set complement
/// }
/// ```
///
/// The generated `struct`s can also be extended with type and trait implementations:
///
/// ```{.rust}
/// use std::fmt;
///
/// bitflags! {
///     flags Flags: u32 {
///         const FLAG_A   = 0x00000001,
///         const FLAG_B   = 0x00000010,
///     }
/// }
///
/// impl Flags {
///     pub fn clear(&mut self) {
///         self.bits = 0;  // The `bits` field can be accessed from within the
///                         // same module where the `bitflags!` macro was invoked.
///     }
/// }
///
/// impl fmt::Show for Flags {
///     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
///         write!(f, "hi!")
///     }
/// }
///
/// fn main() {
///     let mut flags = FLAG_A | FLAG_B;
///     flags.clear();
///     assert!(flags.is_empty());
///     assert_eq!(format!("{}", flags).as_slice(), "hi!");
/// }
/// ```
///
/// # Attributes
///
/// Attributes can be attached to the generated `struct` by placing them
/// before the `flags` keyword.
///
/// # Derived traits
///
/// The `PartialEq` and `Clone` traits are automatically derived for the `struct` using
/// the `deriving` attribute. Additional traits can be derived by providing an
/// explicit `deriving` attribute on `flags`.
///
/// # Operators
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
    ($(#[$attr:meta])* flags $BitFlags:ident: $T:ty {
        $($(#[$Flag_attr:meta])* const $Flag:ident = $value:expr),+
    }) => {
        #[deriving(PartialEq, Eq, Clone, PartialOrd, Ord)]
        $(#[$attr])*
        pub struct $BitFlags {
            bits: $T,
        }

        $($(#[$Flag_attr])* pub const $Flag: $BitFlags = $BitFlags { bits: $value };)+

        impl $BitFlags {
            /// Returns an empty set of flags.
            #[inline]
            pub fn empty() -> $BitFlags {
                $BitFlags { bits: 0 }
            }

            /// Returns the set containing all flags.
            #[inline]
            pub fn all() -> $BitFlags {
                $BitFlags { bits: $($value)|+ }
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
                    ::std::option::None
                } else {
                    ::std::option::Some($BitFlags { bits: bits })
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

        impl core::ops::BitOr<$BitFlags, $BitFlags> for $BitFlags {
            /// Returns the union of the two sets of flags.
            #[inline]
            fn bitor(&self, other: &$BitFlags) -> $BitFlags {
                $BitFlags { bits: self.bits | other.bits }
            }
        }

        impl core::ops::BitXor<$BitFlags, $BitFlags> for $BitFlags {
            /// Returns the left flags, but with all the right flags toggled.
            #[inline]
            fn bitxor(&self, other: &$BitFlags) -> $BitFlags {
                $BitFlags { bits: self.bits ^ other.bits }
            }
        }

        impl core::ops::BitAnd<$BitFlags, $BitFlags> for $BitFlags {
            /// Returns the intersection between the two sets of flags.
            #[inline]
            fn bitand(&self, other: &$BitFlags) -> $BitFlags {
                $BitFlags { bits: self.bits & other.bits }
            }
        }

        impl core::ops::Sub<$BitFlags, $BitFlags> for $BitFlags {
            /// Returns the set difference of the two sets of flags.
            #[inline]
            fn sub(&self, other: &$BitFlags) -> $BitFlags {
                $BitFlags { bits: self.bits & !other.bits }
            }
        }

        impl core::ops::Not<$BitFlags> for $BitFlags {
            /// Returns the complement of this set of flags.
            #[inline]
            fn not(&self) -> $BitFlags {
                $BitFlags { bits: !self.bits } & $BitFlags::all()
            }
        }
    };
    ($(#[$attr:meta])* flags $BitFlags:ident: $T:ty {
        $($(#[$Flag_attr:meta])* const $Flag:ident = $value:expr),+,
    }) => {
        bitflags! {
            $(#[$attr])*
            flags $BitFlags: $T {
                $($(#[$Flag_attr])* const $Flag = $value),+
            }
        }
    };
}
