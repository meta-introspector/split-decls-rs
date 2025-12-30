// Generated macro for macro_328 (macro)
macro_rules! Depcrate_tests_propertiesmacro_328 {
() => {
// Module: crate::tests::properties
// Provides: {"macro_328"}
// Dependencies: {}
prop_compose ! { fn action_strategy () (tid in THREADS , kind in action_kind_strategy ()) -> Action { Action { tid , kind } } }
};
}
