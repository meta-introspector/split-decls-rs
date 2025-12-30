// Generated macro for mk_bin (function)
macro_rules! Depcrate_test_utilsmk_bin {
() => {
// Module: crate::test::utils
// Provides: {"mk_bin"}
// Dependencies: {}
# [cfg (not (unix))] pub fn mk_bin (dir : & Path , path : & str) -> io :: Result < PathBuf > { touch (dir , Path :: new (path) . with_extension (env :: consts :: EXE_EXTENSION) . to_str () . unwrap () ,) }
};
}
