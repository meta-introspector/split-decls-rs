// Generated macro for impl_302 (impl)
macro_rules! Depcrate_unstable_convert_stable_tyimpl_302 {
() => {
// Module: crate::unstable::convert::stable::ty
// Provides: {"impl_302"}
// Dependencies: {}
impl < 'tcx > Stable < 'tcx > for ty :: Generics { type T = crate :: ty :: Generics ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { use crate :: ty :: Generics ; let params : Vec < _ > = self . own_params . iter () . map (| param | param . stable (tables , cx)) . collect () ; let param_def_id_to_index = params . iter () . map (| param | (param . def_id , param . index)) . collect () ; Generics { parent : self . parent . map (| did | tables . generic_def (did)) , parent_count : self . parent_count , params , param_def_id_to_index , has_self : self . has_self , has_late_bound_regions : self . has_late_bound_regions . as_ref () . map (| late_bound_regions | late_bound_regions . stable (tables , cx)) , } } }
};
}
