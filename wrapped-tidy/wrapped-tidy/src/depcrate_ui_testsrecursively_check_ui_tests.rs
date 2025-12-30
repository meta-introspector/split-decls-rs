// Generated macro for recursively_check_ui_tests (function)
macro_rules! Depcrate_ui_testsrecursively_check_ui_tests {
() => {
// Module: crate::ui_tests
// Provides: {"recursively_check_ui_tests"}
// Dependencies: {}
fn recursively_check_ui_tests < 'issues > (bad : & mut bool , path : & Path , allowed_issue_names : & 'issues BTreeSet < & 'issues str > ,) -> BTreeSet < & 'issues str > { let mut remaining_issue_names : BTreeSet < & str > = allowed_issue_names . clone () ; let (ui , ui_fulldeps) = (path . join ("ui") , path . join ("ui-fulldeps")) ; let paths = [ui . as_path () , ui_fulldeps . as_path ()] ; crate :: walk :: walk_no_read (& paths , | _ , _ | false , & mut | entry | { let file_path = entry . path () ; if let Some (ext) = file_path . extension () . and_then (OsStr :: to_str) { check_unexpected_extension (bad , file_path , ext) ; let testname = file_path . file_name () . unwrap () . to_str () . unwrap () . split_once ('.') . unwrap () . 0 ; if ext == "stderr" || ext == "stdout" || ext == "fixed" { check_stray_output_snapshot (bad , file_path , testname) ; check_empty_output_snapshot (bad , file_path) ; } deny_new_nondescriptive_test_names (bad , path , & mut remaining_issue_names , file_path , testname , ext ,) ; } }) ; remaining_issue_names }
};
}
