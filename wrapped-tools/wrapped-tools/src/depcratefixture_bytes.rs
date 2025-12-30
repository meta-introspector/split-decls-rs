// Generated macro for fixture_bytes (function)
macro_rules! Depcratefixture_bytes {
() => {
// Module: crate
// Provides: {"fixture_bytes"}
// Dependencies: {}
# [doc = " Load the fixture from `<crate-root>/tests/fixtures/<path>` and return its data, or _panic_."] pub fn fixture_bytes (path : impl AsRef < Path >) -> Vec < u8 > { fixture_bytes_inner (path , DirectoryRoot :: IntegrationTest) }
};
}
