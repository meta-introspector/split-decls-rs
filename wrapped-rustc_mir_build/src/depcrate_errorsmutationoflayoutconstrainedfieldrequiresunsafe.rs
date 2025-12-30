// Generated macro for MutationOfLayoutConstrainedFieldRequiresUnsafe (struct)
macro_rules! Depcrate_errorsMutationOfLayoutConstrainedFieldRequiresUnsafe {
() => {
// Module: crate::errors
// Provides: {"MutationOfLayoutConstrainedFieldRequiresUnsafe"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (mir_build_mutation_of_layout_constrained_field_requires_unsafe , code = E0133)] # [note] pub (crate) struct MutationOfLayoutConstrainedFieldRequiresUnsafe { # [primary_span] # [label] pub (crate) span : Span , # [subdiagnostic] pub (crate) unsafe_not_inherited_note : Option < UnsafeNotInheritedNote > , }
};
}
