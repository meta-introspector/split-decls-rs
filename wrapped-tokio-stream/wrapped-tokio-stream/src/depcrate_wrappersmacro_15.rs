// Generated macro for macro_15 (macro)
macro_rules! Depcrate_wrappersmacro_15 {
() => {
// Module: crate::wrappers
// Provides: {"macro_15"}
// Dependencies: {}
cfg_signal ! { # [cfg (all (unix , not (loom)))] mod signal_unix ; # [cfg (all (unix , not (loom)))] pub use signal_unix :: SignalStream ; # [cfg (any (windows , docsrs))] mod signal_windows ; # [cfg (any (windows , docsrs))] pub use signal_windows :: { CtrlCStream , CtrlBreakStream } ; }
};
}
