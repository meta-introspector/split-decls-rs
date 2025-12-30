// Generated macro for cfg_has_const_mutex_new (macro)
macro_rules! Depcrate_macros_cfgcfg_has_const_mutex_new {
() => {
// Module: crate::macros::cfg
// Provides: {"cfg_has_const_mutex_new"}
// Dependencies: {}
macro_rules ! cfg_has_const_mutex_new { ($ ($ item : item) *) => { $ (# [cfg (not (all (loom , test)))] $ item) * } }
};
}
