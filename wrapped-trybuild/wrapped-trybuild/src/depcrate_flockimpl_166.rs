// Generated macro for impl_166 (impl)
macro_rules! Depcrate_flockimpl_166 {
() => {
// Module: crate::flock
// Provides: {"impl_166"}
// Dependencies: {}
impl Drop for FileLock { fn drop (& mut self) { match self { FileLock :: NotLocked => { } FileLock :: Locked { path , done } => { done . store (true , Ordering :: Release) ; let _ = fs :: remove_file (path) ; } } } }
};
}
