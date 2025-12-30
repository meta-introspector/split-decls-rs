// Generated macro for fixture_bytes_standalone (function)
macro_rules! Depcratefixture_bytes_standalone {
() => {
// Module: crate
// Provides: {"fixture_bytes_standalone"}
// Dependencies: {}
# [doc = " Like [`scripted_fixture_writable`], but does not prefix the fixture directory with `tests`"] pub fn fixture_bytes_standalone (path : impl AsRef < Path >) -> Vec < u8 > { fixture_bytes_inner (path , DirectoryRoot :: StandaloneTest) }
};
}
