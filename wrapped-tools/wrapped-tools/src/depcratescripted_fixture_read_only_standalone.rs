// Generated macro for scripted_fixture_read_only_standalone (function)
macro_rules! Depcratescripted_fixture_read_only_standalone {
() => {
// Module: crate
// Provides: {"scripted_fixture_read_only_standalone"}
// Dependencies: {}
# [doc = " Like [`scripted_fixture_read_only`], but does not prefix the fixture directory with `tests`"] pub fn scripted_fixture_read_only_standalone (script_name : impl AsRef < Path >) -> Result < PathBuf > { scripted_fixture_read_only_with_args_standalone (script_name , None :: < String >) }
};
}
