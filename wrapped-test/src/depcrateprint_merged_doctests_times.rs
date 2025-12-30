// Generated macro for print_merged_doctests_times (function)
macro_rules! Depcrateprint_merged_doctests_times {
() => {
// Module: crate
// Provides: {"print_merged_doctests_times"}
// Dependencies: {}
# [doc = " Public API used by rustdoc to display the `total` and `compilation` times in the expected"] # [doc = " format."] pub fn print_merged_doctests_times (args : & [String] , total_time : f64 , compilation_time : f64) { let opts = match cli :: parse_opts (args) { Some (Ok (o)) => o , Some (Err (msg)) => { eprintln ! ("error: {msg}") ; process :: exit (ERROR_EXIT_CODE) ; } None => return , } ; let mut formatter = console :: get_formatter (& opts , 0) ; formatter . write_merged_doctests_times (total_time , compilation_time) . unwrap () ; }
};
}
