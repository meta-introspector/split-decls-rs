// Generated macro for test_directory (function)
macro_rules! Depcratetest_directory {
() => {
// Module: crate
// Provides: {"test_directory"}
// Dependencies: {}
# [doc = " Returns a suitable directory to place output for tests within."] # [doc = ""] # [doc = " This tries to pick a location in the `target` directory that can be"] # [doc = " relatively easily debugged if a test goes wrong."] pub fn test_directory (suite_name : & str , gen_name : & str , wit_name : & str) -> PathBuf { let mut me = std :: env :: current_exe () . unwrap () ; me . pop () ; me . pop () ; me . pop () ; me . push (format ! ("{suite_name}-tests")) ; me . push (gen_name) ; me . push (wit_name . replace ("-" , "_")) ; drop (fs :: remove_dir_all (& me)) ; fs :: create_dir_all (& me) . unwrap () ; return me ; }
};
}
