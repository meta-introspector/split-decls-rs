// Generated macro for get_test_files (function)
macro_rules! Depcrate_testget_test_files {
() => {
// Module: crate::test
// Provides: {"get_test_files"}
// Dependencies: {}
fn get_test_files (path : & Path , recursive : bool) -> Vec < PathBuf > { let mut files = vec ! [] ; if path . is_dir () { for entry in fs :: read_dir (path) . expect (& format ! ("couldn't read directory {}" , path . display ())) { let entry = entry . expect ("couldn't get `DirEntry`") ; let path = entry . path () ; if path . is_dir () && recursive { files . append (& mut get_test_files (& path , recursive)) ; } else if path . extension () . map_or (false , | f | f == "rs") && ! is_file_skip (& path) { files . push (path) ; } } } files }
};
}
