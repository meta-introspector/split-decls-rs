// Generated macro for deny_new_top_level_ui_tests (function)
macro_rules! Depcrate_ui_testsdeny_new_top_level_ui_tests {
() => {
// Module: crate::ui_tests
// Provides: {"deny_new_top_level_ui_tests"}
// Dependencies: {}
fn deny_new_top_level_ui_tests (bad : & mut bool , tests_path : & Path) { let top_level_ui_tests = walkdir :: WalkDir :: new (tests_path) . min_depth (1) . max_depth (1) . follow_links (false) . same_file_system (true) . into_iter () . flatten () . filter (| e | { let file_name = e . file_name () ; file_name != ".gitattributes" && file_name != "README.md" }) . filter (| e | ! e . file_type () . is_dir ()) ; for entry in top_level_ui_tests { tidy_error ! (bad , "ui tests should be added under meaningful subdirectories: `{}`" , entry . path () . display ()) } }
};
}
