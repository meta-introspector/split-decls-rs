// Generated macro for impl_199 (impl)
macro_rules! Depcrate_unstable_convert_stable_abiimpl_199 {
() => {
// Module: crate::unstable::convert::stable::abi
// Provides: {"impl_199"}
// Dependencies: {}
impl < 'tcx > Stable < 'tcx > for rustc_abi :: Size { type T = Size ; fn stable (& self , _ : & mut Tables < '_ , BridgeTys > , _ : & CompilerCtxt < '_ , BridgeTys >) -> Self :: T { Size :: from_bits (self . bits_usize ()) } }
};
}
