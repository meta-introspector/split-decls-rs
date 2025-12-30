// Generated macro for impl_300 (impl)
macro_rules! Depcrate_unstable_convert_stable_tyimpl_300 {
() => {
// Module: crate::unstable::convert::stable::ty
// Provides: {"impl_300"}
// Dependencies: {}
impl < 'tcx > Stable < 'tcx > for ty :: TraitDef { type T = crate :: ty :: TraitDecl ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { use crate :: opaque ; use crate :: ty :: TraitDecl ; TraitDecl { def_id : tables . trait_def (self . def_id) , safety : self . safety . stable (tables , cx) , paren_sugar : self . paren_sugar , has_auto_impl : self . has_auto_impl , is_marker : self . is_marker , is_coinductive : self . is_coinductive , skip_array_during_method_dispatch : self . skip_array_during_method_dispatch , skip_boxed_slice_during_method_dispatch : self . skip_boxed_slice_during_method_dispatch , specialization_kind : self . specialization_kind . stable (tables , cx) , must_implement_one_of : self . must_implement_one_of . as_ref () . map (| idents | idents . iter () . map (| ident | opaque (ident)) . collect ()) , implement_via_object : self . implement_via_object , deny_explicit_impl : self . deny_explicit_impl , } } }
};
}
