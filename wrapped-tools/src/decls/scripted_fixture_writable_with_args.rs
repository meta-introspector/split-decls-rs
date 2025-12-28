macro_rules! deps {
    () => {
        DirectoryRoot!();
        ArgsInHash!();
        Result!();
        Creation!();
    };
}

macro_rules! scripted_fixture_writable_with_args {
    () => {
        deps!();
        # [doc = " Like [`scripted_fixture_writable()`], but passes `args` to `script_name` while providing control over"] # [doc = " the way files are created with `mode`."] pub fn scripted_fixture_writable_with_args (script_name : impl AsRef < Path > , args : impl IntoIterator < Item = impl Into < String > > , mode : Creation ,) -> Result < tempfile :: TempDir > { scripted_fixture_writable_with_args_inner (script_name , args , mode , DirectoryRoot :: IntegrationTest , ArgsInHash :: Yes) }
    };
}

scripted_fixture_writable_with_args!();