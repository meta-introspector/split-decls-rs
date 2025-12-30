// Generated macro for impl_296 (impl)
macro_rules! Depcrate_unstable_convert_stable_tyimpl_296 {
() => {
// Module: crate::unstable::convert::stable::ty
// Provides: {"impl_296"}
// Dependencies: {}
impl < 'tcx > Stable < 'tcx > for ty :: ParamConst { type T = crate :: ty :: ParamConst ; fn stable (& self , _ : & mut Tables < '_ , BridgeTys > , _ : & CompilerCtxt < '_ , BridgeTys >) -> Self :: T { use crate :: ty :: ParamConst ; ParamConst { index : self . index , name : self . name . to_string () } } }
};
}
