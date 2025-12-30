// Generated macro for cfg_process_driver (macro)
macro_rules! Depcrate_macros_cfgcfg_process_driver {
() => {
// Module: crate::macros::cfg
// Provides: {"cfg_process_driver"}
// Dependencies: {}
macro_rules ! cfg_process_driver { ($ ($ item : item) *) => { # [cfg (unix)] # [cfg (not (loom))] cfg_process ! { $ ($ item) * } } }
};
}
