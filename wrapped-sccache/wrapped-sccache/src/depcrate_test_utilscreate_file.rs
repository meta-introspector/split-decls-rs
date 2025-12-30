// Generated macro for create_file (function)
macro_rules! Depcrate_test_utilscreate_file {
() => {
// Module: crate::test::utils
// Provides: {"create_file"}
// Dependencies: {}
pub fn create_file < F > (dir : & Path , path : & str , fill_contents : F) -> io :: Result < PathBuf > where F : FnOnce (File) -> io :: Result < () > , { let b = dir . join (path) ; let parent = b . parent () . unwrap () ; fs :: create_dir_all (parent) ? ; let f = fs :: File :: create (& b) ? ; fill_contents (f) ? ; b . canonicalize () }
};
}
