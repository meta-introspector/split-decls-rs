// Generated macro for IncorrectTarget (struct)
macro_rules! Depcrate_errorsIncorrectTarget {
() => {
// Module: crate::errors
// Provides: {"IncorrectTarget"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (passes_incorrect_target , code = E0718)] pub (crate) struct IncorrectTarget < 'a > { # [primary_span] pub span : Span , # [label] pub generics_span : Span , pub name : & 'a str , pub kind : & 'static str , pub num : usize , pub actual_num : usize , pub at_least : bool , }
};
}
