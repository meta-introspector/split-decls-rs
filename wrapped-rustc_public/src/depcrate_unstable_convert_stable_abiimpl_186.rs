// Generated macro for impl_186 (impl)
macro_rules! Depcrate_unstable_convert_stable_abiimpl_186 {
() => {
// Module: crate::unstable::convert::stable::abi
// Provides: {"impl_186"}
// Dependencies: {}
impl < 'tcx > Stable < 'tcx > for rustc_abi :: VariantIdx { type T = VariantIdx ; fn stable (& self , _ : & mut Tables < '_ , BridgeTys > , _ : & CompilerCtxt < '_ , BridgeTys >) -> Self :: T { VariantIdx :: to_val (self . as_usize ()) } }
};
}
