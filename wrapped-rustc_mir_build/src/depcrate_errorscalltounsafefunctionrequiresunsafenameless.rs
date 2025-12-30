// Generated macro for CallToUnsafeFunctionRequiresUnsafeNameless (struct)
macro_rules! Depcrate_errorsCallToUnsafeFunctionRequiresUnsafeNameless {
() => {
// Module: crate::errors
// Provides: {"CallToUnsafeFunctionRequiresUnsafeNameless"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (mir_build_call_to_unsafe_fn_requires_unsafe_nameless , code = E0133)] # [note] pub (crate) struct CallToUnsafeFunctionRequiresUnsafeNameless { # [primary_span] # [label] pub (crate) span : Span , # [subdiagnostic] pub (crate) unsafe_not_inherited_note : Option < UnsafeNotInheritedNote > , }
};
}
