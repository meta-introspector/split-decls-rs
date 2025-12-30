// Generated macro for mk_bin_contents (function)
macro_rules! Depcrate_test_utilsmk_bin_contents {
() => {
// Module: crate::test::utils
// Provides: {"mk_bin_contents"}
// Dependencies: {}
# [cfg (not (unix))] # [allow (dead_code)] pub fn mk_bin_contents < F : FnOnce (File) -> io :: Result < () > > (dir : & Path , path : & str , contents : F ,) -> io :: Result < PathBuf > { create_file (dir , Path :: new (path) . with_extension (env :: consts :: EXE_EXTENSION) . to_str () . unwrap () , contents ,) }
};
}
