// Generated macro for test_formatting (function)
macro_rules! Depcrate_tidytest_formatting {
() => {
// Module: crate::tidy
// Provides: {"test_formatting"}
// Dependencies: {}
# [test] fn test_formatting () { let sh = Shell :: new () . unwrap () ; cmd ! (sh , "cargo fmt -- --check") . run () . unwrap () }
};
}
