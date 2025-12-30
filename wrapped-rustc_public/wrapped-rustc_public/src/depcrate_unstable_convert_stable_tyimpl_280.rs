// Generated macro for impl_280 (impl)
macro_rules! Depcrate_unstable_convert_stable_tyimpl_280 {
() => {
// Module: crate::unstable::convert::stable::ty
// Provides: {"impl_280"}
// Dependencies: {}
impl < 'tcx > Stable < 'tcx > for ty :: FieldDef { type T = crate :: ty :: FieldDef ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { crate :: ty :: FieldDef { def : tables . create_def_id (self . did) , name : self . name . stable (tables , cx) , } } }
};
}
