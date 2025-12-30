// Generated macro for err (macro)
macro_rules! Depcrate_tests_utilerr {
() => {
// Module: crate::tests::util
// Provides: {"err"}
// Dependencies: {}
# [doc = " Create an error from a format!-like syntax."] # [macro_export] macro_rules ! err { ($ ($ tt : tt) *) => { Box ::< dyn error :: Error + Send + Sync >:: from (format ! ($ ($ tt) *)) } }
};
}
