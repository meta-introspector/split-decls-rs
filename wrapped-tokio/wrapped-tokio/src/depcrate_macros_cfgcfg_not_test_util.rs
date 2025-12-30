// Generated macro for cfg_not_test_util (macro)
macro_rules! Depcrate_macros_cfgcfg_not_test_util {
() => {
// Module: crate::macros::cfg
// Provides: {"cfg_not_test_util"}
// Dependencies: {}
macro_rules ! cfg_not_test_util { ($ ($ item : item) *) => { $ (# [cfg (not (feature = "test-util"))] $ item) * } }
};
}
