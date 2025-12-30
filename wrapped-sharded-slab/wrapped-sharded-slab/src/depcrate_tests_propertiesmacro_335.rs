// Generated macro for macro_335 (macro)
macro_rules! Depcrate_tests_propertiesmacro_335 {
() => {
// Module: crate::tests::properties
// Provides: {"macro_335"}
// Dependencies: {}
proptest ! { # [test] fn default_config (actions in prop :: collection :: vec (action_strategy () , ACTIONS)) { run ::< DefaultConfig > (actions) ?; } # [test] fn custom_config (actions in prop :: collection :: vec (action_strategy () , ACTIONS)) { run ::< CustomConfig > (actions) ?; } }
};
}
