// Generated macro for impl_301 (impl)
macro_rules! Depcrate_unstable_convert_stable_tyimpl_301 {
() => {
// Module: crate::unstable::convert::stable::ty
// Provides: {"impl_301"}
// Dependencies: {}
impl < 'tcx > Stable < 'tcx > for ty :: TraitRef < 'tcx > { type T = crate :: ty :: TraitRef ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { use crate :: ty :: TraitRef ; TraitRef :: try_new (tables . trait_def (self . def_id) , self . args . stable (tables , cx)) . unwrap () } }
};
}
