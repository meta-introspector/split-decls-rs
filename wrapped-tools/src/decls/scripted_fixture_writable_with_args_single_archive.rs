macro_rules! deps {
    () => {
        DirectoryRoot!();
        Creation!();
        ArgsInHash!();
        Result!();
    };
}

macro_rules! scripted_fixture_writable_with_args_single_archive {
    () => {
        deps!();
        # [doc = " Like [`scripted_fixture_writable()`], but passes `args` to `script_name` while providing control over"] # [doc = " the way files are created with `mode`."] # [doc = ""] # [doc = " See [`scripted_fixture_read_only_with_args_single_archive()`] for important details on what `single_archive` means."] pub fn scripted_fixture_writable_with_args_single_archive (script_name : impl AsRef < Path > , args : impl IntoIterator < Item = impl Into < String > > , mode : Creation ,) -> Result < tempfile :: TempDir > { scripted_fixture_writable_with_args_inner (script_name , args , mode , DirectoryRoot :: IntegrationTest , ArgsInHash :: No) }
    };
}

scripted_fixture_writable_with_args_single_archive!()