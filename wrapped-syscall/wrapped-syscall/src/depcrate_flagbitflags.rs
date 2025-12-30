// Generated macro for bitflags (macro)
macro_rules! Depcrate_flagbitflags {
() => {
// Module: crate::flag
// Provides: {"bitflags"}
// Dependencies: {}
macro_rules ! bitflags { ($ (# [$ outer : meta]) * pub struct $ BitFlags : ident : $ T : ty { $ ($ (# [$ inner : ident $ ($ args : tt) *]) * const $ Flag : ident = $ value : expr ;) + }) => { inner_bitflags ! { # [derive (PartialEq , Eq , PartialOrd , Ord , Hash , Debug , Clone , Copy , Default)] $ (# [$ outer]) * pub struct $ BitFlags : $ T { $ ($ (# [$ inner $ ($ args) *]) * const $ Flag = $ value ;) + } } impl $ BitFlags { # [deprecated = "use the safe `from_bits_retain` method instead"] pub unsafe fn from_bits_unchecked (bits : $ T) -> Self { Self :: from_bits_retain (bits) } } $ ($ (# [$ inner $ ($ args) *]) * pub const $ Flag : $ BitFlags = $ BitFlags ::$ Flag ;) + } }
};
}
