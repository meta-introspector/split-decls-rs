// Generated macro for impl_141 (impl)
macro_rules! Depcrate_helpersimpl_141 {
() => {
// Module: crate::helpers
// Provides: {"impl_141"}
// Dependencies: {}
impl < T > MaybeSplitAt < T > for [T] { # [inline] fn debug_split_at (& self , mid : usize) -> (& Self , & Self) { self . split_at_checked (mid) . unwrap_or_else (| | { debug_assert ! (false , "debug_split_at: {mid} expected to be in range") ; (self , & []) }) } }
};
}
