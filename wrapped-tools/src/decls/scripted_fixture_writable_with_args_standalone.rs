macro_rules! deps {
    () => {
        Result!();
        Creation!();
        DirectoryRoot!();
        ArgsInHash!();
    };
}

macro_rules! scripted_fixture_writable_with_args_standalone {
    () => {
        deps!();
        # [doc = " Like [`scripted_fixture_writable_with_args`], but does not prefix the fixture directory with `tests`"] pub fn scripted_fixture_writable_with_args_standalone (script_name : & str , args : impl IntoIterator < Item = impl Into < String > > , mode : Creation ,) -> Result < tempfile :: TempDir > { scripted_fixture_writable_with_args_inner (script_name , args , mode , DirectoryRoot :: StandaloneTest , ArgsInHash :: Yes) }
    };
}

scripted_fixture_writable_with_args_standalone!();