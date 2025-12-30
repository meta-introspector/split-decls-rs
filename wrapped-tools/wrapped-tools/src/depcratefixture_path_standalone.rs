// Generated macro for fixture_path_standalone (function)
macro_rules! Depcratefixture_path_standalone {
() => {
// Module: crate
// Provides: {"fixture_path_standalone"}
// Dependencies: {}
# [doc = " Return the path to the `<crate-root>/fixtures/<path>` directory."] pub fn fixture_path_standalone (path : impl AsRef < Path >) -> PathBuf { fixture_path_inner (path , DirectoryRoot :: StandaloneTest) }
};
}
