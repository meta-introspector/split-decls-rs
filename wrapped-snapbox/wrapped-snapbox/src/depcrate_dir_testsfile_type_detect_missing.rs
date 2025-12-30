// Generated macro for file_type_detect_missing (function)
macro_rules! Depcrate_dir_testsfile_type_detect_missing {
() => {
// Module: crate::dir::tests
// Provides: {"file_type_detect_missing"}
// Dependencies: {}
# [test] fn file_type_detect_missing () { let path = std :: path :: Path :: new ("this-should-never-exist") ; dbg ! (path) ; let actual = FileType :: from_path (path) ; assert_eq ! (actual , FileType :: Missing) ; }
};
}
