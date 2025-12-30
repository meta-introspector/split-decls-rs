// Generated macro for impl_187 (impl)
macro_rules! Depcrate_unstable_convert_stable_abiimpl_187 {
() => {
// Module: crate::unstable::convert::stable::abi
// Provides: {"impl_187"}
// Dependencies: {}
impl < 'tcx > Stable < 'tcx > for rustc_abi :: Endian { type T = crate :: target :: Endian ; fn stable (& self , _ : & mut Tables < '_ , BridgeTys > , _ : & CompilerCtxt < '_ , BridgeTys >) -> Self :: T { match self { rustc_abi :: Endian :: Little => crate :: target :: Endian :: Little , rustc_abi :: Endian :: Big => crate :: target :: Endian :: Big , } } }
};
}
