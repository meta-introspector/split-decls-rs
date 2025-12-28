macro_rules! deps {
    () => {
        ArgsInHash!();
        DirectoryRoot!();
        Result!();
    };
}

macro_rules! scripted_fixture_read_only_with_args_standalone {
    () => {
        deps!();
        # [doc = " Like [`scripted_fixture_read_only_with_args()`], but does not prefix the fixture directory with `tests`"] pub fn scripted_fixture_read_only_with_args_standalone (script_name : impl AsRef < Path > , args : impl IntoIterator < Item = impl Into < String > > ,) -> Result < PathBuf > { scripted_fixture_read_only_with_args_inner (script_name , args , None , DirectoryRoot :: StandaloneTest , ArgsInHash :: Yes) }
    };
}

scripted_fixture_read_only_with_args_standalone!()