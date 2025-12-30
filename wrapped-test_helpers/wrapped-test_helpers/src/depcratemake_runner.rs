// Generated macro for make_runner (function)
macro_rules! Depcratemake_runner {
() => {
// Module: crate
// Provides: {"make_runner"}
// Dependencies: {}
# [cfg (miri)] pub fn make_runner () -> proptest :: test_runner :: TestRunner { proptest :: test_runner :: TestRunner :: new (proptest :: test_runner :: Config :: with_cases (4)) }
};
}
