// Generated macro for UnsafeBinderCastRequiresUnsafe (struct)
macro_rules! Depcrate_errorsUnsafeBinderCastRequiresUnsafe {
() => {
// Module: crate::errors
// Provides: {"UnsafeBinderCastRequiresUnsafe"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (mir_build_unsafe_binder_cast_requires_unsafe , code = E0133 ,)] pub (crate) struct UnsafeBinderCastRequiresUnsafe { # [primary_span] # [label] pub (crate) span : Span , # [subdiagnostic] pub (crate) unsafe_not_inherited_note : Option < UnsafeNotInheritedNote > , }
};
}
