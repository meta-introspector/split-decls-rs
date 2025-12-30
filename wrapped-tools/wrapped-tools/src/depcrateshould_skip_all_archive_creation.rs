// Generated macro for should_skip_all_archive_creation (function)
macro_rules! Depcrateshould_skip_all_archive_creation {
() => {
// Module: crate
// Provides: {"should_skip_all_archive_creation"}
// Dependencies: {}
fn should_skip_all_archive_creation () -> bool { cfg ! (windows) || (is_ci :: cached () && env :: var_os ("GIX_TEST_CREATE_ARCHIVES_EVEN_ON_CI") . is_none ()) }
};
}
