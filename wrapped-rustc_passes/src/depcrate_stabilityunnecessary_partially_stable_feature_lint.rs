// Generated macro for unnecessary_partially_stable_feature_lint (function)
macro_rules! Depcrate_stabilityunnecessary_partially_stable_feature_lint {
() => {
// Module: crate::stability
// Provides: {"unnecessary_partially_stable_feature_lint"}
// Dependencies: {}
fn unnecessary_partially_stable_feature_lint (tcx : TyCtxt < '_ > , span : Span , feature : Symbol , implies : Symbol , since : Symbol ,) { tcx . emit_node_span_lint (lint :: builtin :: STABLE_FEATURES , hir :: CRATE_HIR_ID , span , errors :: UnnecessaryPartialStableFeature { span , line : tcx . sess . source_map () . span_extend_to_line (span) , feature , since , implies , } ,) ; }
};
}
