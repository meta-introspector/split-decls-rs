// Generated macro for test_filen_gate (function)
macro_rules! Depcrate_featurestest_filen_gate {
() => {
// Module: crate::features
// Provides: {"test_filen_gate"}
// Dependencies: {}
fn test_filen_gate < 'f > (filen_underscore : & 'f str , features : & mut Features) -> Option < & 'f str > { let prefix = "feature_gate_" ; if let Some (suffix) = filen_underscore . strip_prefix (prefix) { for (n , f) in features . iter_mut () { if suffix == n { f . has_gate_test = true ; return Some (suffix) ; } } } None }
};
}
