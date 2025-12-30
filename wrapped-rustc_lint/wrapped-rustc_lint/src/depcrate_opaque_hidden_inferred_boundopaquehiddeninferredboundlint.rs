// Generated macro for OpaqueHiddenInferredBoundLint (struct)
macro_rules! Depcrate_opaque_hidden_inferred_boundOpaqueHiddenInferredBoundLint {
() => {
// Module: crate::opaque_hidden_inferred_bound
// Provides: {"OpaqueHiddenInferredBoundLint"}
// Dependencies: {}
# [derive (LintDiagnostic)] # [diag (lint_opaque_hidden_inferred_bound)] struct OpaqueHiddenInferredBoundLint < 'tcx > { ty : Ty < 'tcx > , proj_ty : Ty < 'tcx > , # [label (lint_specifically)] assoc_pred_span : Span , # [subdiagnostic] add_bound : Option < AddBound < 'tcx > > , }
};
}
