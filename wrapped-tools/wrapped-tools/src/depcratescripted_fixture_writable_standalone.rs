// Generated macro for scripted_fixture_writable_standalone (function)
macro_rules! Depcratescripted_fixture_writable_standalone {
() => {
// Module: crate
// Provides: {"scripted_fixture_writable_standalone"}
// Dependencies: {}
# [doc = " Like [`scripted_fixture_writable`], but does not prefix the fixture directory with `tests`"] pub fn scripted_fixture_writable_standalone (script_name : & str) -> Result < tempfile :: TempDir > { scripted_fixture_writable_with_args_standalone (script_name , None :: < String > , Creation :: CopyFromReadOnly) }
};
}
