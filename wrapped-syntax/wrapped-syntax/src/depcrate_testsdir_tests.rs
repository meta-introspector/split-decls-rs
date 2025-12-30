// Generated macro for dir_tests (function)
macro_rules! Depcrate_testsdir_tests {
() => {
// Module: crate::tests
// Provides: {"dir_tests"}
// Dependencies: {}
# [doc = " Calls callback `f` with input code and file paths for each `.rs` file in `test_data_dir`"] # [doc = " subdirectories defined by `paths`."] # [doc = ""] # [doc = " If the content of the matching output file differs from the output of `f()`"] # [doc = " the test will fail."] # [doc = ""] # [doc = " If there is no matching output file it will be created and filled with the"] # [doc = " output of `f()`, but the test will fail."] fn dir_tests < F > (test_data_dir : & Path , paths : & [& str] , outfile_extension : & str , f : F) where F : Fn (& str , & Path) -> String , { for (path , input_code) in collect_rust_files (test_data_dir , paths) { let actual = f (& input_code , & path) ; let path = path . with_extension (outfile_extension) ; expect_file ! [path] . assert_eq (& actual) } }
};
}
