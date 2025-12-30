// Generated macro for ConstantTimeEq (trait)
macro_rules! DepcrateConstantTimeEq {
() => {
// Module: crate
// Provides: {"ConstantTimeEq"}
// Dependencies: {}
# [doc = " An `Eq`-like trait that produces a `Choice` instead of a `bool`."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use subtle::ConstantTimeEq;"] # [doc = " let x: u8 = 5;"] # [doc = " let y: u8 = 13;"] # [doc = ""] # [doc = " assert_eq!(x.ct_eq(&y).unwrap_u8(), 0);"] # [doc = " assert_eq!(x.ct_eq(&x).unwrap_u8(), 1);"] # [doc = " ```"] # [allow (unused_attributes)] pub trait ConstantTimeEq { # [doc = " Determine if two items are equal."] # [doc = ""] # [doc = " The `ct_eq` function should execute in constant time."] # [doc = ""] # [doc = " # Returns"] # [doc = ""] # [doc = " * `Choice(1u8)` if `self == other`;"] # [doc = " * `Choice(0u8)` if `self != other`."] # [inline] # [allow (unused_attributes)] fn ct_eq (& self , other : & Self) -> Choice ; # [doc = " Determine if two items are NOT equal."] # [doc = ""] # [doc = " The `ct_ne` function should execute in constant time."] # [doc = ""] # [doc = " # Returns"] # [doc = ""] # [doc = " * `Choice(0u8)` if `self == other`;"] # [doc = " * `Choice(1u8)` if `self != other`."] # [inline] fn ct_ne (& self , other : & Self) -> Choice { ! self . ct_eq (other) } }
};
}
