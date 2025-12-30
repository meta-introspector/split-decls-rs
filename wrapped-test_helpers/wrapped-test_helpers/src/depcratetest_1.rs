// Generated macro for test_1 (function)
macro_rules! Depcratetest_1 {
() => {
// Module: crate
// Provides: {"test_1"}
// Dependencies: {}
# [doc = " Test a function that takes a single value."] pub fn test_1 < A : core :: fmt :: Debug + DefaultStrategy > (f : & dyn Fn (A) -> proptest :: test_runner :: TestCaseResult ,) { let mut runner = make_runner () ; runner . run (& A :: default_strategy () , f) . unwrap () ; }
};
}
