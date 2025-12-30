// Generated macro for reparse_fuzz_tests (function)
macro_rules! Depcrate_testsreparse_fuzz_tests {
() => {
// Module: crate::tests
// Provides: {"reparse_fuzz_tests"}
// Dependencies: {}
# [test] fn reparse_fuzz_tests () { for (_ , text) in collect_rust_files (& test_data_dir () , & ["reparse/fuzz-failures"]) { let check = fuzz :: CheckReparse :: from_data (text . as_bytes ()) . unwrap () ; check . run () ; } }
};
}
