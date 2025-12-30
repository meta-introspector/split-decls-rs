// Generated macro for daemonize (function)
macro_rules! Depcrate_utildaemonize {
() => {
// Module: crate::util
// Provides: {"daemonize"}
// Dependencies: {}
# [doc = " This is a no-op on Windows."] # [cfg (windows)] pub fn daemonize () -> Result < () > { Ok (()) }
};
}
