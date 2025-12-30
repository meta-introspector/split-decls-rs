// Generated macro for test_2 (function)
macro_rules! Depcratetest_2 {
() => {
// Module: crate
// Provides: {"test_2"}
// Dependencies: {}
# [doc = " Test a function that takes two values."] pub fn test_2 < A : core :: fmt :: Debug + DefaultStrategy , B : core :: fmt :: Debug + DefaultStrategy > (f : & dyn Fn (A , B) -> proptest :: test_runner :: TestCaseResult ,) { let mut runner = make_runner () ; runner . run (& (A :: default_strategy () , B :: default_strategy ()) , | (a , b) | { f (a , b) }) . unwrap () ; }
};
}
