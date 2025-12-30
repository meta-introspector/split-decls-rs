// Generated macro for check_trailing_ws (function)
macro_rules! Depcrate_tidycheck_trailing_ws {
() => {
// Module: crate::tidy
// Provides: {"check_trailing_ws"}
// Dependencies: {}
fn check_trailing_ws (path : & Path , text : & str) { if is_exclude_dir (path , & ["test_data"]) { return ; } for (line_number , line) in text . lines () . enumerate () { if line . chars () . last () . is_some_and (char :: is_whitespace) { panic ! ("Trailing whitespace in {} at line {}" , path . display () , line_number + 1) } } }
};
}
