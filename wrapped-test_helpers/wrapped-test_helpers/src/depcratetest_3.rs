// Generated macro for test_3 (function)
macro_rules! Depcratetest_3 {
() => {
// Module: crate
// Provides: {"test_3"}
// Dependencies: {}
# [doc = " Test a function that takes two values."] pub fn test_3 < A : core :: fmt :: Debug + DefaultStrategy , B : core :: fmt :: Debug + DefaultStrategy , C : core :: fmt :: Debug + DefaultStrategy , > (f : & dyn Fn (A , B , C) -> proptest :: test_runner :: TestCaseResult ,) { let mut runner = make_runner () ; runner . run (& (A :: default_strategy () , B :: default_strategy () , C :: default_strategy () ,) , | (a , b , c) | f (a , b , c) ,) . unwrap () ; }
};
}
