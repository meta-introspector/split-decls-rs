// Generated macro for fixture_path (function)
macro_rules! Depcratefixture_path {
() => {
// Module: crate
// Provides: {"fixture_path"}
// Dependencies: {}
# [doc = " Return the path to the `<crate-root>/tests/fixtures/<path>` directory."] pub fn fixture_path (path : impl AsRef < Path >) -> PathBuf { fixture_path_inner (path , DirectoryRoot :: IntegrationTest) }
};
}
