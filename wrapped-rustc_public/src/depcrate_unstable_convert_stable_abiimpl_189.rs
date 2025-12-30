// Generated macro for impl_189 (impl)
macro_rules! Depcrate_unstable_convert_stable_abiimpl_189 {
() => {
// Module: crate::unstable::convert::stable::abi
// Provides: {"impl_189"}
// Dependencies: {}
impl < 'tcx > Stable < 'tcx > for rustc_abi :: Layout < 'tcx > { type T = Layout ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { tables . layout_id (cx . lift (* self) . unwrap ()) } }
};
}
