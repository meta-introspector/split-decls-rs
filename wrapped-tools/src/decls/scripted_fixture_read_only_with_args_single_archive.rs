macro_rules! deps {
    () => {
        DirectoryRoot!();
        ArgsInHash!();
        Result!();
    };
}

macro_rules! scripted_fixture_read_only_with_args_single_archive {
    () => {
        deps!();
        # [doc = " Like `scripted_fixture_read_only()`], but passes `args` to `script_name`."] # [doc = ""] # [doc = " Also, don't add a suffix to the archive name as `args` are platform dependent, none-deterministic,"] # [doc = " or otherwise don't influence the content of the archive."] # [doc = " Note that this also means that `args` won't be used to control the hash of the archive itself."] # [doc = ""] # [doc = " Sometimes, this should be combined with adding the archive name to `.gitignore` to prevent its creation"] # [doc = " in the first place."] # [doc = ""] # [doc = " Note that suffixing archives by default helps to learn what calls are made, and forces the author to"] # [doc = " think about what should be done to get it right."] pub fn scripted_fixture_read_only_with_args_single_archive (script_name : impl AsRef < Path > , args : impl IntoIterator < Item = impl Into < String > > ,) -> Result < PathBuf > { scripted_fixture_read_only_with_args_inner (script_name , args , None , DirectoryRoot :: IntegrationTest , ArgsInHash :: No) }
    };
}

scripted_fixture_read_only_with_args_single_archive!();