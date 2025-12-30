// Generated macro for file_type_detect_file (function)
macro_rules! Depcrate_dir_testsfile_type_detect_file {
() => {
// Module: crate::dir::tests
// Provides: {"file_type_detect_file"}
// Dependencies: {}
# [test] fn file_type_detect_file () { let path = std :: path :: Path :: new (env ! ("CARGO_MANIFEST_DIR")) . join ("Cargo.toml") ; dbg ! (& path) ; let actual = FileType :: from_path (& path) ; assert_eq ! (actual , FileType :: File) ; }
};
}
