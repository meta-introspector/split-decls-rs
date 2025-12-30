// Generated macro for FileLock (enum)
macro_rules! Depcrate_flockFileLock {
() => {
// Module: crate::flock
// Provides: {"FileLock"}
// Dependencies: {}
enum FileLock { NotLocked , Locked { path : PathBuf , done : Arc < AtomicBool > , } , }
};
}
