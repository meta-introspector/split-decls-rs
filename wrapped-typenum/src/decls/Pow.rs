macro_rules! deps {
    () => {
        Exp!();
    };
}

macro_rules! Pow {
    () => {
        deps!();
        # [doc = " A **type operator** that provides exponentiation by repeated squaring."] # [doc = ""] # [doc = " # Example"] # [doc = " ```rust"] # [doc = " use typenum::{Integer, Pow, N3, P3};"] # [doc = ""] # [doc = " assert_eq!(<N3 as Pow<P3>>::Output::to_i32(), -27);"] # [doc = " ```"] pub trait Pow < Exp > { # [doc = " The result of the exponentiation."] type Output ; # [doc = " This function isn't used in this crate, but may be useful for others."] # [doc = " It is implemented for primitives."] # [doc = ""] # [doc = " # Example"] # [doc = " ```rust"] # [doc = " use typenum::{Pow, U3};"] # [doc = ""] # [doc = " let a = 7u32.powi(U3::new());"] # [doc = " let b = 7u32.pow(3);"] # [doc = " assert_eq!(a, b);"] # [doc = ""] # [doc = " let x = 3.0.powi(U3::new());"] # [doc = " let y = 27.0;"] # [doc = " assert_eq!(x, y);"] # [doc = " ```"] fn powi (self , exp : Exp) -> Self :: Output ; }
    };
}

Pow!();