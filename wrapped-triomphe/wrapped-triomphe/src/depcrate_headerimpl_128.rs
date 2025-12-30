// Generated macro for impl_128 (impl)
macro_rules! Depcrate_headerimpl_128 {
() => {
// Module: crate::header
// Provides: {"impl_128"}
// Dependencies: {}
impl < T : Copy > From < & [T] > for Arc < [T] > { fn from (slice : & [T]) -> Self { Arc :: from_header_and_slice (() , slice) . into () } }
};
}
