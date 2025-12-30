// Generated macro for make_owned_test (function)
macro_rules! Depcratemake_owned_test {
() => {
// Module: crate
// Provides: {"make_owned_test"}
// Dependencies: {}
# [doc = " Clones static values for putting into a dynamic vector, which test_main()"] # [doc = " needs to hand out ownership of tests to parallel test runners."] # [doc = ""] # [doc = " This will panic when fed any dynamic tests, because they cannot be cloned."] fn make_owned_test (test : & & TestDescAndFn) -> TestDescAndFn { match test . testfn { StaticTestFn (f) => TestDescAndFn { testfn : StaticTestFn (f) , desc : test . desc . clone () } , StaticBenchFn (f) => TestDescAndFn { testfn : StaticBenchFn (f) , desc : test . desc . clone () } , _ => panic ! ("non-static tests passed to test::test_main_static") , } }
};
}
