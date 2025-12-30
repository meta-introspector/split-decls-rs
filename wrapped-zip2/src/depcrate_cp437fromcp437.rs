// Generated macro for FromCp437 (trait)
macro_rules! Depcrate_cp437FromCp437 {
() => {
// Module: crate::cp437
// Provides: {"FromCp437"}
// Dependencies: {}
# [doc = " Trait to convert IBM codepage 437 to the target type"] pub trait FromCp437 { # [doc = " Target type"] type Target ; # [doc = " Function that does the conversion from cp437."] # [doc = " Generally allocations will be avoided if all data falls into the ASCII range."] # [allow (clippy :: wrong_self_convention)] fn from_cp437 (self) -> Self :: Target ; }
};
}
