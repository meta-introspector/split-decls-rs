// Generated macro for touch (function)
macro_rules! Depcrate_test_utilstouch {
() => {
// Module: crate::test::utils
// Provides: {"touch"}
// Dependencies: {}
pub fn touch (dir : & Path , path : & str) -> io :: Result < PathBuf > { create_file (dir , path , | _f | Ok (())) }
};
}
