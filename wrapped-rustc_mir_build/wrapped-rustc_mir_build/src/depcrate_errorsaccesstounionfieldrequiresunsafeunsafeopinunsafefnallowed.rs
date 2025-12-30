// Generated macro for AccessToUnionFieldRequiresUnsafeUnsafeOpInUnsafeFnAllowed (struct)
macro_rules! Depcrate_errorsAccessToUnionFieldRequiresUnsafeUnsafeOpInUnsafeFnAllowed {
() => {
// Module: crate::errors
// Provides: {"AccessToUnionFieldRequiresUnsafeUnsafeOpInUnsafeFnAllowed"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (mir_build_union_field_requires_unsafe_unsafe_op_in_unsafe_fn_allowed , code = E0133)] # [note] pub (crate) struct AccessToUnionFieldRequiresUnsafeUnsafeOpInUnsafeFnAllowed { # [primary_span] # [label] pub (crate) span : Span , # [subdiagnostic] pub (crate) unsafe_not_inherited_note : Option < UnsafeNotInheritedNote > , }
};
}
