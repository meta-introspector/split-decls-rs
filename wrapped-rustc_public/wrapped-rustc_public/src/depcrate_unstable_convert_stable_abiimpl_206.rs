// Generated macro for impl_206 (impl)
macro_rules! Depcrate_unstable_convert_stable_abiimpl_206 {
() => {
// Module: crate::unstable::convert::stable::abi
// Provides: {"impl_206"}
// Dependencies: {}
impl < 'tcx > Stable < 'tcx > for rustc_abi :: WrappingRange { type T = WrappingRange ; fn stable (& self , _ : & mut Tables < '_ , BridgeTys > , _ : & CompilerCtxt < '_ , BridgeTys >) -> Self :: T { WrappingRange { start : self . start , end : self . end } } }
};
}
