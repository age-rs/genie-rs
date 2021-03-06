#[macro_export]
/// Create an infallible TryInto implementation for an ID container type that returns its contained number as the given target type.
///
///
/// ## Example
///
/// ```rust
/// use genie_support::infallible_try_into;
/// use std::convert::TryInto;
/// struct Container(u16);
/// infallible_try_into!(Container, u32);
/// let num: u32 = Container(12).try_into().unwrap();
/// assert_eq!(num, 12u32);
/// ```
macro_rules! infallible_try_into {
    ($from:ident, $to:ty) => {
        impl ::std::convert::TryFrom<$from> for $to {
            type Error = ::std::convert::Infallible;
            fn try_from(n: $from) -> ::std::result::Result<Self, Self::Error> {
                use ::std::convert::TryInto;
                n.0.try_into()
            }
        }
    };
}

#[macro_export]
/// Create a TryInto implementation for an ID container type that tries to returns its contained number as the given target type.
///
/// ## Example
///
/// ```rust
/// use genie_support::fallible_try_into;
/// use std::convert::{TryFrom, TryInto};
/// struct Container(u16);
/// fallible_try_into!(Container, i16);
/// let num: i16 = Container(12).try_into().unwrap();
/// assert_eq!(num, 12i16);
/// assert!(i16::try_from(Container(50000u16)).is_err());
/// ```
macro_rules! fallible_try_into {
    ($from:ident, $to:ty) => {
        impl ::std::convert::TryFrom<$from> for $to {
            type Error = ::std::num::TryFromIntError;
            #[inline]
            fn try_from(n: $from) -> ::std::result::Result<Self, Self::Error> {
                use ::std::convert::TryInto;
                n.0.try_into()
            }
        }
    };
}

#[macro_export]
/// Create a TryFrom implementation for an ID container type that tries to wrap the given number
/// type into the container.
///
/// ## Example
///
/// ```rust
/// use genie_support::fallible_try_from;
/// use std::convert::{TryFrom, TryInto};
/// #[derive(Debug, PartialEq, Eq)]
/// struct Container(u16);
/// fallible_try_from!(Container, i16);
/// assert_eq!(Container::try_from(1i16).unwrap(), Container(1));
/// assert!(Container::try_from(-1i16).is_err());
/// ```
macro_rules! fallible_try_from {
    ($to:ty, $from:ident) => {
        impl ::std::convert::TryFrom<$from> for $to {
            type Error = ::std::num::TryFromIntError;
            #[inline]
            fn try_from(n: $from) -> ::std::result::Result<Self, Self::Error> {
                use ::std::convert::TryInto;
                n.try_into().map(Self)
            }
        }
    };
}

#[macro_export]
/// Check if two 32 bit floating point numbers are equal, with some error.
///
/// ```rust
/// use genie_support::f32_eq;
/// let zero = 0.0;
/// assert!(f32_eq!(zero, 0.0));
/// assert!(!f32_eq!(zero, 1.0));
/// ```
macro_rules! f32_eq {
    ($left:expr, $right:expr) => {
        f32::abs($left - $right) < std::f32::EPSILON
    };
}

#[macro_export]
/// Check if two 32 bit floating point numbers are not equal, with some error.
///
/// ```rust
/// use genie_support::f32_neq;
/// let zero = 0.0;
/// assert!(f32_neq!(zero, 1.0));
/// assert!(!f32_neq!(zero, 0.0));
/// ```
macro_rules! f32_neq {
    ($left:expr, $right:expr) => {
        f32::abs($left - $right) > std::f32::EPSILON
    };
}
