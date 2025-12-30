// Generated macro for impl_162 (impl)
macro_rules! Depcrate_flockimpl_162 {
() => {
// Module: crate::flock
// Provides: {"impl_162"}
// Dependencies: {}
impl Lock { pub fn acquire (path : impl AsRef < Path >) -> Result < Self > { Ok (Lock { intraprocess_guard : Guard :: acquire () , lockfile : FileLock :: acquire (path) ? , }) } }
};
}
