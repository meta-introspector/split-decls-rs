macro_rules! deps {
    () => {
        Unsigned!();
    };
}

macro_rules! Gcd {
    () => {
        deps!();
        # [doc = " A **type operator** that computes the [greatest common divisor][gcd] of `Self` and `Rhs`."] # [doc = ""] # [doc = " [gcd]: https://en.wikipedia.org/wiki/Greatest_common_divisor"] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " use typenum::{Gcd, Unsigned, U12, U8};"] # [doc = ""] # [doc = " assert_eq!(<U12 as Gcd<U8>>::Output::to_i32(), 4);"] # [doc = " ```"] pub trait Gcd < Rhs > { # [doc = " The greatest common divisor."] type Output ; }
    };
}

Gcd!();