// Generated macro for UseOfInlineAssemblyRequiresUnsafe (struct)
macro_rules! Depcrate_errorsUseOfInlineAssemblyRequiresUnsafe {
() => {
// Module: crate::errors
// Provides: {"UseOfInlineAssemblyRequiresUnsafe"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (mir_build_inline_assembly_requires_unsafe , code = E0133)] # [note] pub (crate) struct UseOfInlineAssemblyRequiresUnsafe { # [primary_span] # [label] pub (crate) span : Span , # [subdiagnostic] pub (crate) unsafe_not_inherited_note : Option < UnsafeNotInheritedNote > , }
};
}
