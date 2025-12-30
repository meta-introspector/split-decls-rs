// Generated macro for impl_325 (impl)
macro_rules! Depcrate_unstable_convert_stable_tyimpl_325 {
() => {
// Module: crate::unstable::convert::stable::ty
// Provides: {"impl_325"}
// Dependencies: {}
impl < 'tcx > Stable < 'tcx > for ty :: AssocItem { type T = crate :: ty :: AssocItem ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { crate :: ty :: AssocItem { def_id : tables . assoc_def (self . def_id) , kind : self . kind . stable (tables , cx) , container : self . container . stable (tables , cx) , } } }
};
}
