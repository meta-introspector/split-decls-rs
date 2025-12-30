// Generated macro for IntegerType (enum)
macro_rules! Depcrate_abiIntegerType {
() => {
// Module: crate::abi
// Provides: {"IntegerType"}
// Dependencies: {}
# [derive (Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash , Debug , Serialize)] pub enum IntegerType { # [doc = " Pointer-sized integer type, i.e. `isize` and `usize`."] Pointer { # [doc = " Signedness. e.g. `true` for `isize`"] is_signed : bool , } , # [doc = " Fixed-sized integer type, e.g. `i8`, `u32`, `i128`."] Fixed { # [doc = " Length of this integer type. e.g. `IntegerLength::I8` for `u8`."] length : IntegerLength , # [doc = " Signedness. e.g. `false` for `u8`"] is_signed : bool , } , }
};
}
