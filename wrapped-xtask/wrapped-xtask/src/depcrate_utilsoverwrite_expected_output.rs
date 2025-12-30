// Generated macro for overwrite_expected_output (function)
macro_rules! Depcrate_utilsoverwrite_expected_output {
() => {
// Module: crate::utils
// Provides: {"overwrite_expected_output"}
// Dependencies: {}
pub fn overwrite_expected_output (name : & str , contents : & [u8] , is_test : bool) -> anyhow :: Result < () > { let file = expected_output_path (name , is_test) ; let path = Path :: new (& file) ; fs :: write (path , contents) . with_context (| | { format ! ("Failed to overwrite expected output data to {}" , path . to_str () . unwrap_or ("(non-Unicode path)")) }) }
};
}
