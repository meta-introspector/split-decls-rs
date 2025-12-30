// Generated macro for impl_297 (impl)
macro_rules! Depcrate_unstable_convert_stable_tyimpl_297 {
() => {
// Module: crate::unstable::convert::stable::ty
// Provides: {"impl_297"}
// Dependencies: {}
impl < 'tcx > Stable < 'tcx > for ty :: ParamTy { type T = crate :: ty :: ParamTy ; fn stable (& self , _ : & mut Tables < '_ , BridgeTys > , _ : & CompilerCtxt < '_ , BridgeTys >) -> Self :: T { use crate :: ty :: ParamTy ; ParamTy { index : self . index , name : self . name . to_string () } } }
};
}
