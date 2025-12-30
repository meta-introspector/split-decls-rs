// Generated macro for BuiltinTypeAliasBounds (struct)
macro_rules! Depcrate_lintsBuiltinTypeAliasBounds {
() => {
// Module: crate::lints
// Provides: {"BuiltinTypeAliasBounds"}
// Dependencies: {}
pub (crate) struct BuiltinTypeAliasBounds < 'hir > { pub in_where_clause : bool , pub label : Span , pub enable_feat_help : bool , pub suggestions : Vec < (Span , String) > , pub preds : & 'hir [hir :: WherePredicate < 'hir >] , pub ty : Option < & 'hir hir :: Ty < 'hir > > , }
};
}
