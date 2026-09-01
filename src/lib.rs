#![no_std]

/// Implements an empty `Debug` trait.
/// It does not require fields to implement `Debug`.
#[macro_export]
macro_rules! nodebug {
    ($typ:ty) => {
        impl ::core::fmt::Debug for $typ {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::core::write!(f, "{}::debug", ::core::stringify!($typ))
            }
        }
    };
}