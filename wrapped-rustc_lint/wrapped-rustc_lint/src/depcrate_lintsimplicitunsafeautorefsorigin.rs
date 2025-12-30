// Generated macro for ImplicitUnsafeAutorefsOrigin (enum)
macro_rules! Depcrate_lintsImplicitUnsafeAutorefsOrigin {
() => {
// Module: crate::lints
// Provides: {"ImplicitUnsafeAutorefsOrigin"}
// Dependencies: {}
# [derive (Subdiagnostic)] pub (crate) enum ImplicitUnsafeAutorefsOrigin < 'a > { # [note (lint_autoref)] Autoref { # [primary_span] autoref_span : Span , autoref_ty : Ty < 'a > , } , # [note (lint_overloaded_deref)] OverloadedDeref , }
};
}
