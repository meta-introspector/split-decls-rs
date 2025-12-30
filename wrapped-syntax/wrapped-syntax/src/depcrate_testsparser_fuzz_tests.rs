// Generated macro for parser_fuzz_tests (function)
macro_rules! Depcrate_testsparser_fuzz_tests {
() => {
// Module: crate::tests
// Provides: {"parser_fuzz_tests"}
// Dependencies: {}
# [test] fn parser_fuzz_tests () { for (_ , text) in collect_rust_files (& test_data_dir () , & ["parser/fuzz-failures"]) { fuzz :: check_parser (& text) } }
};
}
