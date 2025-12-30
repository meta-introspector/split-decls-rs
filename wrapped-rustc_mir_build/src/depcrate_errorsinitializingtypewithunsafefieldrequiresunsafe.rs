// Generated macro for InitializingTypeWithUnsafeFieldRequiresUnsafe (struct)
macro_rules! Depcrate_errorsInitializingTypeWithUnsafeFieldRequiresUnsafe {
() => {
// Module: crate::errors
// Provides: {"InitializingTypeWithUnsafeFieldRequiresUnsafe"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (mir_build_initializing_type_with_unsafe_field_requires_unsafe , code = E0133)] # [note] pub (crate) struct InitializingTypeWithUnsafeFieldRequiresUnsafe { # [primary_span] # [label] pub (crate) span : Span , # [subdiagnostic] pub (crate) unsafe_not_inherited_note : Option < UnsafeNotInheritedNote > , }
};
}
