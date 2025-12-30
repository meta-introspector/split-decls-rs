// Generated macro for load_expected_output (function)
macro_rules! Depcrate_utilsload_expected_output {
() => {
// Module: crate::utils
// Provides: {"load_expected_output"}
// Dependencies: {}
pub fn load_expected_output (name : & str , is_test : bool) -> anyhow :: Result < String > { let path = expected_output_path (name , is_test) ; fs :: read_to_string (& path) . with_context (| | { format ! ("Failed to load expected output data from {}" , path . to_str () . unwrap_or ("(non-Unicode path)")) }) }
};
}
