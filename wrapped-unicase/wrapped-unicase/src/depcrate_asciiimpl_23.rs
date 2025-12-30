// Generated macro for impl_23 (impl)
macro_rules! Depcrate_asciiimpl_23 {
() => {
// Module: crate::ascii
// Provides: {"impl_23"}
// Dependencies: {}
impl < T : AsRef < str > > PartialOrd for Ascii < T > { # [inline] fn partial_cmp (& self , other : & Self) -> Option < Ordering > { Some (self . cmp (other)) } }
};
}
