// Generated macro for AccessToUnionFieldRequiresUnsafe (struct)
macro_rules! Depcrate_errorsAccessToUnionFieldRequiresUnsafe {
() => {
// Module: crate::errors
// Provides: {"AccessToUnionFieldRequiresUnsafe"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (mir_build_union_field_requires_unsafe , code = E0133)] # [note] pub (crate) struct AccessToUnionFieldRequiresUnsafe { # [primary_span] # [label] pub (crate) span : Span , # [subdiagnostic] pub (crate) unsafe_not_inherited_note : Option < UnsafeNotInheritedNote > , }
};
}
