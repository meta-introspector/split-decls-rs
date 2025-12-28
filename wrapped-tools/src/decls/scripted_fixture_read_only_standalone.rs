macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! scripted_fixture_read_only_standalone {
    () => {
        deps!();
        # [doc = " Like [`scripted_fixture_read_only`], but does not prefix the fixture directory with `tests`"] pub fn scripted_fixture_read_only_standalone (script_name : impl AsRef < Path >) -> Result < PathBuf > { scripted_fixture_read_only_with_args_standalone (script_name , None :: < String >) }
    };
}

scripted_fixture_read_only_standalone!()