macro_rules! deps {
    () => {
        Integer!();
    };
}

macro_rules! Abs {
    () => {
        deps!();
        # [doc = " A **type operator** that returns the absolute value."] # [doc = ""] # [doc = " # Example"] # [doc = " ```rust"] # [doc = " use typenum::{Abs, Integer, N5};"] # [doc = ""] # [doc = " assert_eq!(<N5 as Abs>::Output::to_i32(), 5);"] # [doc = " ```"] pub trait Abs { # [doc = " The absolute value."] type Output ; }
    };
}

Abs!()