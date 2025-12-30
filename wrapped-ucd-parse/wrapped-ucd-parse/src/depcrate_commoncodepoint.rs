// Generated macro for Codepoint (struct)
macro_rules! Depcrate_commonCodepoint {
() => {
// Module: crate::common
// Provides: {"Codepoint"}
// Dependencies: {}
# [doc = " A single Unicode codepoint."] # [doc = ""] # [doc = " This type's string representation is a hexadecimal number. It is guaranteed"] # [doc = " to be in the range `[0, 10FFFF]`."] # [doc = ""] # [doc = " Note that unlike Rust's `char` type, this may be a surrogate codepoint."] # [derive (Clone , Copy , Debug , Default , Eq , Hash , PartialEq , PartialOrd , Ord ,)] pub struct Codepoint (u32) ;
};
}
