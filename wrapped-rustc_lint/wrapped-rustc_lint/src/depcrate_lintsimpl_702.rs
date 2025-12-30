// Generated macro for impl_702 (impl)
macro_rules! Depcrate_lintsimpl_702 {
() => {
// Module: crate::lints
// Provides: {"impl_702"}
// Dependencies: {}
impl < 'a > LintDiagnostic < 'a , () > for BuiltinTypeAliasBounds < '_ > { fn decorate_lint < 'b > (self , diag : & 'b mut Diag < 'a , () >) { diag . primary_message (if self . in_where_clause { fluent :: lint_builtin_type_alias_bounds_where_clause } else { fluent :: lint_builtin_type_alias_bounds_param_bounds }) ; diag . span_label (self . label , fluent :: lint_builtin_type_alias_bounds_label) ; diag . note (fluent :: lint_builtin_type_alias_bounds_limitation_note) ; if self . enable_feat_help { diag . help (fluent :: lint_builtin_type_alias_bounds_enable_feat_help) ; } let mut collector = ShorthandAssocTyCollector { qselves : Vec :: new () } ; if let Some (ty) = self . ty { collector . visit_ty_unambig (ty) ; } let affect_object_lifetime_defaults = self . preds . iter () . filter (| pred | pred . kind . in_where_clause () == self . in_where_clause) . any (| pred | TypeAliasBounds :: affects_object_lifetime_defaults (pred)) ; let applicability = if ! collector . qselves . is_empty () || affect_object_lifetime_defaults { Applicability :: MaybeIncorrect } else { Applicability :: MachineApplicable } ; diag . arg ("count" , self . suggestions . len ()) ; diag . multipart_suggestion (fluent :: lint_suggestion , self . suggestions , applicability) ; for qself in collector . qselves { diag . multipart_suggestion (fluent :: lint_builtin_type_alias_bounds_qualify_assoc_tys_sugg , vec ! [(qself . shrink_to_lo () , "<" . into ()) , (qself . shrink_to_hi () , " as /* Trait */>" . into ()) ,] , Applicability :: HasPlaceholders ,) ; } } }
};
}
