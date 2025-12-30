// Generated macro for mangle_typeid_for_trait_ref (function)
macro_rules! Depcrate_v0mangle_typeid_for_trait_ref {
() => {
// Module: crate::v0
// Provides: {"mangle_typeid_for_trait_ref"}
// Dependencies: {}
pub (super) fn mangle_typeid_for_trait_ref < 'tcx > (tcx : TyCtxt < 'tcx > , trait_ref : ty :: ExistentialTraitRef < 'tcx > ,) -> String { let mut p = V0SymbolMangler { tcx , start_offset : 0 , is_exportable : false , paths : FxHashMap :: default () , types : FxHashMap :: default () , consts : FxHashMap :: default () , binders : vec ! [] , out : String :: new () , } ; p . print_def_path (trait_ref . def_id , & []) . unwrap () ; std :: mem :: take (& mut p . out) }
};
}
