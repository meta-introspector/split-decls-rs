// Generated macro for scripted_fixture_read_only_with_args_standalone_single_archive (function)
macro_rules! Depcratescripted_fixture_read_only_with_args_standalone_single_archive {
() => {
// Module: crate
// Provides: {"scripted_fixture_read_only_with_args_standalone_single_archive"}
// Dependencies: {}
# [doc = " Like [`scripted_fixture_read_only_with_args_standalone()`], only has a single archive."] pub fn scripted_fixture_read_only_with_args_standalone_single_archive (script_name : impl AsRef < Path > , args : impl IntoIterator < Item = impl Into < String > > ,) -> Result < PathBuf > { scripted_fixture_read_only_with_args_inner (script_name , args , None , DirectoryRoot :: StandaloneTest , ArgsInHash :: No) }
};
}
