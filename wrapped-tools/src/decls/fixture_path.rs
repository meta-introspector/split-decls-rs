macro_rules! deps {
    () => {
        DirectoryRoot!();
    };
}

macro_rules! fixture_path {
    () => {
        deps!();
        # [doc = " Return the path to the `<crate-root>/tests/fixtures/<path>` directory."] pub fn fixture_path (path : impl AsRef < Path >) -> PathBuf { fixture_path_inner (path , DirectoryRoot :: IntegrationTest) }
    };
}

fixture_path!();