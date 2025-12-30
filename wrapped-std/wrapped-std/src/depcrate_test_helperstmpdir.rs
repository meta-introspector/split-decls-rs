// Generated macro for tmpdir (function)
macro_rules! Depcrate_test_helperstmpdir {
() => {
// Module: crate::test_helpers
// Provides: {"tmpdir"}
// Dependencies: {}
# [track_caller] pub fn tmpdir () -> TempDir { let p = env :: temp_dir () ; let mut r = test_rng () ; let ret = p . join (& format ! ("rust-{}" , r . next_u32 ())) ; fs :: create_dir (& ret) . unwrap () ; TempDir (ret) }
};
}
