macro_rules! deps {
    () => {
        DirectoryRoot!();
        ArgsInHash!();
        Result!();
    };
}

macro_rules! scripted_fixture_read_only_with_args_standalone_single_archive {
    () => {
        deps!();
        # [doc = " Like [`scripted_fixture_read_only_with_args_standalone()`], only has a single archive."] pub fn scripted_fixture_read_only_with_args_standalone_single_archive (script_name : impl AsRef < Path > , args : impl IntoIterator < Item = impl Into < String > > ,) -> Result < PathBuf > { scripted_fixture_read_only_with_args_inner (script_name , args , None , DirectoryRoot :: StandaloneTest , ArgsInHash :: No) }
    };
}

scripted_fixture_read_only_with_args_standalone_single_archive!()