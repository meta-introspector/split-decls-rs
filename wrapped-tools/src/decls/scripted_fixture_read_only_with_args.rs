macro_rules! deps {
    () => {
        DirectoryRoot!();
        ArgsInHash!();
        Result!();
    };
}

macro_rules! scripted_fixture_read_only_with_args {
    () => {
        deps!();
        # [doc = " Like `scripted_fixture_read_only()`], but passes `args` to `script_name`."] pub fn scripted_fixture_read_only_with_args (script_name : impl AsRef < Path > , args : impl IntoIterator < Item = impl Into < String > > ,) -> Result < PathBuf > { scripted_fixture_read_only_with_args_inner (script_name , args , None , DirectoryRoot :: IntegrationTest , ArgsInHash :: Yes) }
    };
}

scripted_fixture_read_only_with_args!()