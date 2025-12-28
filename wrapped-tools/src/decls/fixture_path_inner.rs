macro_rules! deps {
    () => {
        DirectoryRoot!();
    };
}

macro_rules! fixture_path_inner {
    () => {
        deps!();
        # [doc = " Return the path to the `<crate-root>/tests/fixtures/<path>` directory."] fn fixture_path_inner (path : impl AsRef < Path > , root : DirectoryRoot) -> PathBuf { match root { DirectoryRoot :: StandaloneTest => PathBuf :: from ("fixtures") . join (path . as_ref ()) , DirectoryRoot :: IntegrationTest => PathBuf :: from ("tests") . join ("fixtures") . join (path . as_ref ()) , } }
    };
}

fixture_path_inner!()