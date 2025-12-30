// Generated macro for parse_test_cases (function)
macro_rules! Depcrateparse_test_cases {
() => {
// Module: crate
// Provides: {"parse_test_cases"}
// Dependencies: {}
pub fn parse_test_cases () -> Cases { serde_json :: from_str (TEST_VECTORS_JSON) . expect ("failed to parse test_vectors.json") }
};
}
