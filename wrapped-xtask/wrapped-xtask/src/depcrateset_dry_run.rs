// Generated macro for set_dry_run (function)
macro_rules! Depcrateset_dry_run {
() => {
// Module: crate
// Provides: {"set_dry_run"}
// Dependencies: {}
pub fn set_dry_run (yes : bool) { DRY_RUN . store (yes , Ordering :: Relaxed) }
};
}
