// Generated macro for DerefOfRawPointerRequiresUnsafe (struct)
macro_rules! Depcrate_errorsDerefOfRawPointerRequiresUnsafe {
() => {
// Module: crate::errors
// Provides: {"DerefOfRawPointerRequiresUnsafe"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (mir_build_deref_raw_pointer_requires_unsafe , code = E0133)] # [note] pub (crate) struct DerefOfRawPointerRequiresUnsafe { # [primary_span] # [label] pub (crate) span : Span , # [subdiagnostic] pub (crate) unsafe_not_inherited_note : Option < UnsafeNotInheritedNote > , }
};
}
