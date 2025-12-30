// Generated macro for UseOfMutableStaticRequiresUnsafe (struct)
macro_rules! Depcrate_errorsUseOfMutableStaticRequiresUnsafe {
() => {
// Module: crate::errors
// Provides: {"UseOfMutableStaticRequiresUnsafe"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (mir_build_mutable_static_requires_unsafe , code = E0133)] # [note] pub (crate) struct UseOfMutableStaticRequiresUnsafe { # [primary_span] # [label] pub (crate) span : Span , # [subdiagnostic] pub (crate) unsafe_not_inherited_note : Option < UnsafeNotInheritedNote > , }
};
}
