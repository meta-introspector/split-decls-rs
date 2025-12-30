// Generated macro for cfg_not_process_driver (macro)
macro_rules! Depcrate_macros_cfgcfg_not_process_driver {
() => {
// Module: crate::macros::cfg
// Provides: {"cfg_not_process_driver"}
// Dependencies: {}
macro_rules ! cfg_not_process_driver { ($ ($ item : item) *) => { $ (# [cfg (not (all (unix , not (loom) , feature = "process")))] $ item) * } }
};
}
