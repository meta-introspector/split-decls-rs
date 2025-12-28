macro_rules! deps {
    () => {
        DirectoryRoot!();
    };
}

macro_rules! fixture_bytes_standalone {
    () => {
        deps!();
        # [doc = " Like [`scripted_fixture_writable`], but does not prefix the fixture directory with `tests`"] pub fn fixture_bytes_standalone (path : impl AsRef < Path >) -> Vec < u8 > { fixture_bytes_inner (path , DirectoryRoot :: StandaloneTest) }
    };
}

fixture_bytes_standalone!();