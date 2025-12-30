// Generated macro for impl_316 (impl)
macro_rules! Depcrate_unstable_convert_stable_tyimpl_316 {
() => {
// Module: crate::unstable::convert::stable::ty
// Provides: {"impl_316"}
// Dependencies: {}
impl < 'tcx > Stable < 'tcx > for ty :: Region < 'tcx > { type T = crate :: ty :: Region ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { Region { kind : self . kind () . stable (tables , cx) } } }
};
}
