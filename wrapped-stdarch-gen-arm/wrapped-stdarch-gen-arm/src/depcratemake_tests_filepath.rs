// Generated macro for make_tests_filepath (function)
macro_rules! Depcratemake_tests_filepath {
() => {
// Module: crate
// Provides: {"make_tests_filepath"}
// Dependencies: {}
fn make_tests_filepath (in_filepath : & Path , out_dirpath : & Path) -> PathBuf { make_filepath (in_filepath , out_dirpath , | name : & str | { format ! ("ld_st_tests_{name}.rs") }) }
};
}
