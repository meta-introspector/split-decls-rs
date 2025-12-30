// Generated macro for DerefOfRawPointerRequiresUnsafeUnsafeOpInUnsafeFnAllowed (struct)
macro_rules! Depcrate_errorsDerefOfRawPointerRequiresUnsafeUnsafeOpInUnsafeFnAllowed {
() => {
// Module: crate::errors
// Provides: {"DerefOfRawPointerRequiresUnsafeUnsafeOpInUnsafeFnAllowed"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (mir_build_deref_raw_pointer_requires_unsafe_unsafe_op_in_unsafe_fn_allowed , code = E0133)] # [note] pub (crate) struct DerefOfRawPointerRequiresUnsafeUnsafeOpInUnsafeFnAllowed { # [primary_span] # [label] pub (crate) span : Span , # [subdiagnostic] pub (crate) unsafe_not_inherited_note : Option < UnsafeNotInheritedNote > , }
};
}
