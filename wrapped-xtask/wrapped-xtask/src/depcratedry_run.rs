// Generated macro for dry_run (function)
macro_rules! Depcratedry_run {
() => {
// Module: crate
// Provides: {"dry_run"}
// Dependencies: {}
fn dry_run () -> Option < & 'static str > { let dry_run = DRY_RUN . load (Ordering :: Relaxed) ; if dry_run { Some ("--dry-run") } else { None } }
};
}
