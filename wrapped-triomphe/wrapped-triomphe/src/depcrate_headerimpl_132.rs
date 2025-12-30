// Generated macro for impl_132 (impl)
macro_rules! Depcrate_headerimpl_132 {
() => {
// Module: crate::header
// Provides: {"impl_132"}
// Dependencies: {}
impl < T > From < Vec < T > > for Arc < [T] > { fn from (v : Vec < T >) -> Self { Arc :: from_header_and_vec (() , v) . into () } }
};
}
