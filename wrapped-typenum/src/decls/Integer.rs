macro_rules! Integer {
    () => {
        # [doc = " The **marker trait** for compile time signed integers."] # [doc = ""] # [doc = " # Example"] # [doc = " ```rust"] # [doc = " use typenum::{Integer, P3};"] # [doc = ""] # [doc = " assert_eq!(P3::to_i32(), 3);"] # [doc = " assert_eq!(P3::I32, 3);"] # [doc = " ```"] pub trait Integer : Sealed + Copy + Default + 'static { # [allow (missing_docs)] const I8 : i8 ; # [allow (missing_docs)] const I16 : i16 ; # [allow (missing_docs)] const I32 : i32 ; # [allow (missing_docs)] const I64 : i64 ; # [cfg (feature = "i128")] # [allow (missing_docs)] const I128 : i128 ; # [allow (missing_docs)] const ISIZE : isize ; # [allow (missing_docs)] fn to_i8 () -> i8 ; # [allow (missing_docs)] fn to_i16 () -> i16 ; # [allow (missing_docs)] fn to_i32 () -> i32 ; # [allow (missing_docs)] fn to_i64 () -> i64 ; # [cfg (feature = "i128")] # [allow (missing_docs)] fn to_i128 () -> i128 ; # [allow (missing_docs)] fn to_isize () -> isize ; }
    };
}

Integer!()