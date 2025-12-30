// Generated macro for unnecessary_stable_feature_lint (function)
macro_rules! Depcrate_stabilityunnecessary_stable_feature_lint {
() => {
// Module: crate::stability
// Provides: {"unnecessary_stable_feature_lint"}
// Dependencies: {}
fn unnecessary_stable_feature_lint (tcx : TyCtxt < '_ > , span : Span , feature : Symbol , mut since : Symbol ,) { if since . as_str () == VERSION_PLACEHOLDER { since = sym :: env_CFG_RELEASE ; } tcx . emit_node_span_lint (lint :: builtin :: STABLE_FEATURES , hir :: CRATE_HIR_ID , span , errors :: UnnecessaryStableFeature { feature , since } ,) ; }
};
}
