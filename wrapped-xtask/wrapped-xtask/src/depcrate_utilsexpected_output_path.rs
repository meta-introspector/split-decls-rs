// Generated macro for expected_output_path (function)
macro_rules! Depcrate_utilsexpected_output_path {
() => {
// Module: crate::utils
// Provides: {"expected_output_path"}
// Dependencies: {}
fn expected_output_path (name : & str , is_test : bool) -> PathBuf { const PROJECT_DIR : & str = "firmware/qemu" ; let mut path = PathBuf :: from (PROJECT_DIR) ; if is_test { path . push ("tests") } else { path . push ("src") ; path . push ("bin") ; } ; path . push (name) ; path . set_extension ("out") ; path }
};
}
