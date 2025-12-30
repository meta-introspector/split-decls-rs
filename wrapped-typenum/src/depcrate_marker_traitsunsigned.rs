// Generated macro for Unsigned (trait)
macro_rules! Depcrate_marker_traitsUnsigned {
() => {
// Module: crate::marker_traits
// Provides: {"Unsigned"}
// Dependencies: {}
# [doc = " The **marker trait** for compile time unsigned integers."] # [doc = ""] # [doc = " # Example"] # [doc = " ```rust"] # [doc = " use typenum::{Unsigned, U3};"] # [doc = ""] # [doc = " assert_eq!(U3::to_u32(), 3);"] # [doc = " assert_eq!(U3::I32, 3);"] # [doc = " ```"] pub trait Unsigned : Sealed + Copy + Default + 'static { # [allow (missing_docs)] const U8 : u8 ; # [allow (missing_docs)] const U16 : u16 ; # [allow (missing_docs)] const U32 : u32 ; # [allow (missing_docs)] const U64 : u64 ; # [cfg (feature = "i128")] # [allow (missing_docs)] const U128 : u128 ; # [allow (missing_docs)] const USIZE : usize ; # [allow (missing_docs)] const I8 : i8 ; # [allow (missing_docs)] const I16 : i16 ; # [allow (missing_docs)] const I32 : i32 ; # [allow (missing_docs)] const I64 : i64 ; # [cfg (feature = "i128")] # [allow (missing_docs)] const I128 : i128 ; # [allow (missing_docs)] const ISIZE : isize ; # [allow (missing_docs)] fn to_u8 () -> u8 ; # [allow (missing_docs)] fn to_u16 () -> u16 ; # [allow (missing_docs)] fn to_u32 () -> u32 ; # [allow (missing_docs)] fn to_u64 () -> u64 ; # [cfg (feature = "i128")] # [allow (missing_docs)] fn to_u128 () -> u128 ; # [allow (missing_docs)] fn to_usize () -> usize ; # [allow (missing_docs)] fn to_i8 () -> i8 ; # [allow (missing_docs)] fn to_i16 () -> i16 ; # [allow (missing_docs)] fn to_i32 () -> i32 ; # [allow (missing_docs)] fn to_i64 () -> i64 ; # [cfg (feature = "i128")] # [allow (missing_docs)] fn to_i128 () -> i128 ; # [allow (missing_docs)] fn to_isize () -> isize ; }
};
}
