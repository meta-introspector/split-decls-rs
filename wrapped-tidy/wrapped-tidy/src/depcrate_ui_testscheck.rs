// Generated macro for check (function)
macro_rules! Depcrate_ui_testscheck {
() => {
// Module: crate::ui_tests
// Provides: {"check"}
// Dependencies: {}
pub fn check (root_path : & Path , bless : bool , bad : & mut bool) { let path = & root_path . join ("tests") ; let mut prev_line = "" ; let mut is_sorted = true ; let allowed_issue_names : BTreeSet < _ > = include_str ! ("issues.txt") . strip_prefix (ISSUES_TXT_HEADER) . unwrap () . lines () . inspect (| & line | { if prev_line > line { is_sorted = false ; } prev_line = line ; }) . collect () ; if ! is_sorted && ! bless { tidy_error ! (bad , "`src/tools/tidy/src/issues.txt` is not in order, mostly because you modified it manually,
            please only update it with command `x test tidy --bless`") ; } deny_new_top_level_ui_tests (bad , & path . join ("ui")) ; let remaining_issue_names = recursively_check_ui_tests (bad , path , & allowed_issue_names) ; if bless && (! remaining_issue_names . is_empty () || ! is_sorted) { let tidy_src = root_path . join ("src/tools/tidy/src") ; let blessed_issues_path = tidy_src . join ("issues_blessed.txt") ; let mut blessed_issues_txt = fs :: File :: create (& blessed_issues_path) . unwrap () ; blessed_issues_txt . write_all (ISSUES_TXT_HEADER . as_bytes ()) . unwrap () ; for filename in allowed_issue_names . difference (& remaining_issue_names) { writeln ! (blessed_issues_txt , "{filename}") . unwrap () ; } let old_issues_path = tidy_src . join ("issues.txt") ; fs :: rename (blessed_issues_path , old_issues_path) . unwrap () ; } else { for file_name in remaining_issue_names { let mut p = PathBuf :: from (path) ; p . push (file_name) ; tidy_error ! (bad , "file `{}` no longer exists and should be removed from the exclusions in `src/tools/tidy/src/issues.txt`" , p . display ()) ; } } }
};
}
