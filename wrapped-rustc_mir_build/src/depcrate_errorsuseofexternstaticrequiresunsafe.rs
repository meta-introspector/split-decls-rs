// Generated macro for UseOfExternStaticRequiresUnsafe (struct)
macro_rules! Depcrate_errorsUseOfExternStaticRequiresUnsafe {
() => {
// Module: crate::errors
// Provides: {"UseOfExternStaticRequiresUnsafe"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (mir_build_extern_static_requires_unsafe , code = E0133)] # [note] pub (crate) struct UseOfExternStaticRequiresUnsafe { # [primary_span] # [label] pub (crate) span : Span , # [subdiagnostic] pub (crate) unsafe_not_inherited_note : Option < UnsafeNotInheritedNote > , }
};
}
