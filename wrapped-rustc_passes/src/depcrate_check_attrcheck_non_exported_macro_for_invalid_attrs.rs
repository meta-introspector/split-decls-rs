// Generated macro for check_non_exported_macro_for_invalid_attrs (function)
macro_rules! Depcrate_check_attrcheck_non_exported_macro_for_invalid_attrs {
() => {
// Module: crate::check_attr
// Provides: {"check_non_exported_macro_for_invalid_attrs"}
// Dependencies: {}
fn check_non_exported_macro_for_invalid_attrs (tcx : TyCtxt < '_ > , item : & Item < '_ >) { let attrs = tcx . hir_attrs (item . hir_id ()) ; if let Some (attr_span) = find_attr ! (attrs , AttributeKind :: Inline (i , span) if ! matches ! (i , InlineAttr :: Force { .. }) => * span) { tcx . dcx () . emit_err (errors :: NonExportedMacroInvalidAttrs { attr_span }) ; } }
};
}
