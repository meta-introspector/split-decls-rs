// Generated macro for macro_173 (macro)
macro_rules! Depcrate_iomacro_173 {
() => {
// Module: crate::io
// Provides: {"macro_173"}
// Dependencies: {}
cfg_net_unix ! { mod async_fd ; pub mod unix { # ! [doc = " Asynchronous IO structures specific to Unix-like operating systems."] pub use super :: async_fd :: { AsyncFd , AsyncFdTryNewError , AsyncFdReadyGuard , AsyncFdReadyMutGuard , TryIoError } ; } }
};
}
