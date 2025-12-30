// Generated macro for file_type_detect_dir (function)
macro_rules! Depcrate_dir_testsfile_type_detect_dir {
() => {
// Module: crate::dir::tests
// Provides: {"file_type_detect_dir"}
// Dependencies: {}
# [test] fn file_type_detect_dir () { let path = std :: path :: Path :: new (env ! ("CARGO_MANIFEST_DIR")) ; dbg ! (path) ; let actual = FileType :: from_path (path) ; assert_eq ! (actual , FileType :: Dir) ; }
};
}
