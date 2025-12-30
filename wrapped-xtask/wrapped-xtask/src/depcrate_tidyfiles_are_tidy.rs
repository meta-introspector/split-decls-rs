// Generated macro for files_are_tidy (function)
macro_rules! Depcrate_tidyfiles_are_tidy {
() => {
// Module: crate::tidy
// Provides: {"files_are_tidy"}
// Dependencies: {}
fn files_are_tidy (sh : & Shell) { let files = list_files (& project_root () . join ("crates")) ; let mut tidy_docs = TidyDocs :: default () ; let mut tidy_marks = TidyMarks :: default () ; for path in files { let extension = path . extension () . unwrap_or_default () . to_str () . unwrap_or_default () ; match extension { "rs" => { let text = sh . read_file (& path) . unwrap () ; check_test_attrs (& path , & text) ; check_trailing_ws (& path , & text) ; tidy_docs . visit (& path , & text) ; tidy_marks . visit (& path , & text) ; } "toml" => { let text = sh . read_file (& path) . unwrap () ; check_cargo_toml (& path , text) ; } _ => () , } } tidy_docs . finish () ; tidy_marks . finish () ; }
};
}
