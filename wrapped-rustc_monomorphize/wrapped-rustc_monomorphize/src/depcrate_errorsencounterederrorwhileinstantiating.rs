// Generated macro for EncounteredErrorWhileInstantiating (struct)
macro_rules! Depcrate_errorsEncounteredErrorWhileInstantiating {
() => {
// Module: crate::errors
// Provides: {"EncounteredErrorWhileInstantiating"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (monomorphize_encountered_error_while_instantiating)] pub (crate) struct EncounteredErrorWhileInstantiating < 'tcx > { # [primary_span] pub span : Span , pub kind : & 'static str , pub instance : Instance < 'tcx > , }
};
}
