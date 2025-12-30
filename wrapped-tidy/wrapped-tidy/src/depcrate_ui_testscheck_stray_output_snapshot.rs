// Generated macro for check_stray_output_snapshot (function)
macro_rules! Depcrate_ui_testscheck_stray_output_snapshot {
() => {
// Module: crate::ui_tests
// Provides: {"check_stray_output_snapshot"}
// Dependencies: {}
fn check_stray_output_snapshot (bad : & mut bool , file_path : & Path , testname : & str) { if ! file_path . with_file_name (testname) . with_extension ("rs") . exists () && ! testname . contains ("ignore-tidy") { tidy_error ! (bad , "Stray file with UI testing output: {:?}" , file_path) ; } }
};
}
