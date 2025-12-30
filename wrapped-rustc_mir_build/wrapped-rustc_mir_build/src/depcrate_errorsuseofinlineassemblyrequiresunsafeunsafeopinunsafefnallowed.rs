// Generated macro for UseOfInlineAssemblyRequiresUnsafeUnsafeOpInUnsafeFnAllowed (struct)
macro_rules! Depcrate_errorsUseOfInlineAssemblyRequiresUnsafeUnsafeOpInUnsafeFnAllowed {
() => {
// Module: crate::errors
// Provides: {"UseOfInlineAssemblyRequiresUnsafeUnsafeOpInUnsafeFnAllowed"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (mir_build_inline_assembly_requires_unsafe_unsafe_op_in_unsafe_fn_allowed , code = E0133)] # [note] pub (crate) struct UseOfInlineAssemblyRequiresUnsafeUnsafeOpInUnsafeFnAllowed { # [primary_span] # [label] pub (crate) span : Span , # [subdiagnostic] pub (crate) unsafe_not_inherited_note : Option < UnsafeNotInheritedNote > , }
};
}
