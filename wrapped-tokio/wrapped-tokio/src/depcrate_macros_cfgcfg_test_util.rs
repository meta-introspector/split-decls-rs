// Generated macro for cfg_test_util (macro)
macro_rules! Depcrate_macros_cfgcfg_test_util {
() => {
// Module: crate::macros::cfg
// Provides: {"cfg_test_util"}
// Dependencies: {}
macro_rules ! cfg_test_util { ($ ($ item : item) *) => { $ (# [cfg (feature = "test-util")] # [cfg_attr (docsrs , doc (cfg (feature = "test-util")))] $ item) * } }
};
}
