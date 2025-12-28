macro_rules! deps {
    () => {
        ArgsInHash!();
        DirectoryRoot!();
        Result!();
        Creation!();
    };
}

macro_rules! scripted_fixture_writable_with_args_standalone_single_archive {
    () => {
        deps!();
        # [doc = " Like [`scripted_fixture_writable_with_args`], but does not prefix the fixture directory with `tests`"] # [doc = ""] # [doc = " See [`scripted_fixture_read_only_with_args_single_archive()`] for important details on what `single_archive` means."] pub fn scripted_fixture_writable_with_args_standalone_single_archive (script_name : & str , args : impl IntoIterator < Item = impl Into < String > > , mode : Creation ,) -> Result < tempfile :: TempDir > { scripted_fixture_writable_with_args_inner (script_name , args , mode , DirectoryRoot :: StandaloneTest , ArgsInHash :: No) }
    };
}

scripted_fixture_writable_with_args_standalone_single_archive!()