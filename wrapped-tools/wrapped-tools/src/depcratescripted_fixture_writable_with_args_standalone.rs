// Generated macro for scripted_fixture_writable_with_args_standalone (function)
macro_rules! Depcratescripted_fixture_writable_with_args_standalone {
() => {
// Module: crate
// Provides: {"scripted_fixture_writable_with_args_standalone"}
// Dependencies: {}
# [doc = " Like [`scripted_fixture_writable_with_args`], but does not prefix the fixture directory with `tests`"] pub fn scripted_fixture_writable_with_args_standalone (script_name : & str , args : impl IntoIterator < Item = impl Into < String > > , mode : Creation ,) -> Result < tempfile :: TempDir > { scripted_fixture_writable_with_args_inner (script_name , args , mode , DirectoryRoot :: StandaloneTest , ArgsInHash :: Yes) }
};
}
