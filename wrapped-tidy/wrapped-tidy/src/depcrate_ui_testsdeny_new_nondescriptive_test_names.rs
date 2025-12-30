// Generated macro for deny_new_nondescriptive_test_names (function)
macro_rules! Depcrate_ui_testsdeny_new_nondescriptive_test_names {
() => {
// Module: crate::ui_tests
// Provides: {"deny_new_nondescriptive_test_names"}
// Dependencies: {}
fn deny_new_nondescriptive_test_names (bad : & mut bool , path : & Path , remaining_issue_names : & mut BTreeSet < & str > , file_path : & Path , testname : & str , ext : & str ,) { if ext == "rs" && let Some (test_name) = static_regex ! (r"^issues?[-_]?(\d{3,})") . captures (testname) { let stripped_path = file_path . strip_prefix (path) . unwrap () . to_str () . unwrap () . replace (std :: path :: MAIN_SEPARATOR_STR , "/") ; if ! remaining_issue_names . remove (stripped_path . as_str ()) && ! stripped_path . starts_with ("ui/issues/") { tidy_error ! (bad , "file `tests/{stripped_path}` must begin with a descriptive name, consider `{{reason}}-issue-{issue_n}.rs`" , issue_n = & test_name [1] ,) ; } } }
};
}
