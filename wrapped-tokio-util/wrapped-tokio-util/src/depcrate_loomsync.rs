// Generated macro for sync (module)
macro_rules! Depcrate_loomsync {
() => {
// Module: crate::loom
// Provides: {"sync"}
// Dependencies: {}
pub (crate) mod sync { # [cfg (all (test , loom))] pub (crate) use loom :: sync :: { Arc , Mutex , MutexGuard } ; # [cfg (not (all (test , loom)))] pub (crate) use std :: sync :: { Arc , Mutex , MutexGuard } ; }
};
}
