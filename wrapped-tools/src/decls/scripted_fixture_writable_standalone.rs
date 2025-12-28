macro_rules! deps {
    () => {
        Result!();
        Creation!();
    };
}

macro_rules! scripted_fixture_writable_standalone {
    () => {
        deps!();
        # [doc = " Like [`scripted_fixture_writable`], but does not prefix the fixture directory with `tests`"] pub fn scripted_fixture_writable_standalone (script_name : & str) -> Result < tempfile :: TempDir > { scripted_fixture_writable_with_args_standalone (script_name , None :: < String > , Creation :: CopyFromReadOnly) }
    };
}

scripted_fixture_writable_standalone!()