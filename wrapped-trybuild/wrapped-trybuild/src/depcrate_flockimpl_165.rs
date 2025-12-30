// Generated macro for impl_165 (impl)
macro_rules! Depcrate_flockimpl_165 {
() => {
// Module: crate::flock
// Provides: {"impl_165"}
// Dependencies: {}
impl Drop for Lock { fn drop (& mut self) { let Lock { intraprocess_guard , lockfile , } = self ; * lockfile = FileLock :: NotLocked ; * intraprocess_guard = Guard :: NotLocked ; } }
};
}
