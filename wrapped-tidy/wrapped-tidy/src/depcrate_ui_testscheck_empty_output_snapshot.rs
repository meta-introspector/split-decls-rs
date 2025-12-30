// Generated macro for check_empty_output_snapshot (function)
macro_rules! Depcrate_ui_testscheck_empty_output_snapshot {
() => {
// Module: crate::ui_tests
// Provides: {"check_empty_output_snapshot"}
// Dependencies: {}
fn check_empty_output_snapshot (bad : & mut bool , file_path : & Path) { if let Ok (metadata) = fs :: metadata (file_path) && metadata . len () == 0 { tidy_error ! (bad , "Empty file with UI testing output: {:?}" , file_path) ; } }
};
}
