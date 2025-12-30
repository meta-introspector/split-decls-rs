// Generated macro for InitializingTypeWithRequiresUnsafe (struct)
macro_rules! Depcrate_errorsInitializingTypeWithRequiresUnsafe {
() => {
// Module: crate::errors
// Provides: {"InitializingTypeWithRequiresUnsafe"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (mir_build_initializing_type_with_requires_unsafe , code = E0133)] # [note] pub (crate) struct InitializingTypeWithRequiresUnsafe { # [primary_span] # [label] pub (crate) span : Span , # [subdiagnostic] pub (crate) unsafe_not_inherited_note : Option < UnsafeNotInheritedNote > , }
};
}
