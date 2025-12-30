// Generated macro for SliceKind (enum)
macro_rules! Depcrate_constructorSliceKind {
() => {
// Module: crate::constructor
// Provides: {"SliceKind"}
// Dependencies: {}
# [derive (Copy , Clone , Debug , PartialEq , Eq)] pub enum SliceKind { # [doc = " Patterns of length `n` (`[x, y]`)."] FixedLen (usize) , # [doc = " Patterns using the `..` notation (`[x, .., y]`)."] # [doc = " Captures any array constructor of `length >= i + j`."] # [doc = " In the case where `array_len` is `Some(_)`,"] # [doc = " this indicates that we only care about the first `i` and the last `j` values of the array,"] # [doc = " and everything in between is a wildcard `_`."] VarLen (usize , usize) , }
};
}
