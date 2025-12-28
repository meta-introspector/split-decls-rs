macro_rules! deps {
    () => {
        DirectoryRoot!();
    };
}

macro_rules! fixture_path_standalone {
    () => {
        deps!();
        # [doc = " Return the path to the `<crate-root>/fixtures/<path>` directory."] pub fn fixture_path_standalone (path : impl AsRef < Path >) -> PathBuf { fixture_path_inner (path , DirectoryRoot :: StandaloneTest) }
    };
}

fixture_path_standalone!();