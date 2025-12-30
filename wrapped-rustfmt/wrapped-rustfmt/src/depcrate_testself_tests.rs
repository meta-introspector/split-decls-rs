// Generated macro for self_tests (function)
macro_rules! Depcrate_testself_tests {
() => {
// Module: crate::test
// Provides: {"self_tests"}
// Dependencies: {}
# [nightly_only_test] # [test] fn self_tests () { init_log () ; let mut files = get_test_files (Path :: new ("tests") , false) ; let bin_directories = vec ! ["cargo-fmt" , "git-rustfmt" , "bin" , "format-diff"] ; for dir in bin_directories { let mut path = PathBuf :: from ("src") ; path . push (dir) ; path . push ("main.rs") ; files . push (path) ; } let external_crates = vec ! ["check_diff" , "config_proc_macro"] ; for external_crate in external_crates { let mut path = PathBuf :: from (external_crate) ; path . push ("src") ; let directory = fs :: read_dir (& path) . unwrap () ; let search_files = directory . filter_map (| file | { file . ok () . and_then (| f | { let name = f . file_name () ; if matches ! (name . as_os_str () . to_str () , Some ("main.rs" | "lib.rs")) { Some (f . path ()) } else { None } }) }) ; for file in search_files { files . push (file) ; } } files . push (PathBuf :: from ("src/lib.rs")) ; let (reports , count , fails) = check_files (files , & Some (PathBuf :: from ("rustfmt.toml"))) ; let mut warnings = 0 ; println ! ("Ran {count} self tests.") ; assert_eq ! (fails , 0 , "{fails} self tests failed") ; for format_report in reports { println ! ("{}" , FormatReportFormatterBuilder :: new (& format_report) . build ()) ; warnings += format_report . warning_count () ; } assert_eq ! (warnings , 0 , "Rustfmt's code generated {warnings} warnings") ; }
};
}
