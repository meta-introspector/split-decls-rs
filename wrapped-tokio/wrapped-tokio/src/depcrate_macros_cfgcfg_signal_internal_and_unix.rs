// Generated macro for cfg_signal_internal_and_unix (macro)
macro_rules! Depcrate_macros_cfgcfg_signal_internal_and_unix {
() => {
// Module: crate::macros::cfg
// Provides: {"cfg_signal_internal_and_unix"}
// Dependencies: {}
macro_rules ! cfg_signal_internal_and_unix { ($ ($ item : item) *) => { # [cfg (unix)] cfg_signal_internal ! { $ ($ item) * } } }
};
}
