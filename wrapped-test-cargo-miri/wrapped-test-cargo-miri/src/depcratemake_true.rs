// Generated macro for make_true (function)
macro_rules! Depcratemake_true {
() => {
// Module: crate
// Provides: {"make_true"}
// Dependencies: {}
# [doc = " Doc-test test"] # [doc = ""] # [doc = " ```rust"] # [doc = " assert!(cargo_miri_test::make_true());"] # [doc = " ```"] # [doc = ""] # [doc = " `no_run` test:"] # [doc = ""] # [doc = " ```rust,no_run"] # [doc = " assert!(!cargo_miri_test::make_true());"] # [doc = " ```"] # [doc = ""] # [doc = " `compile_fail` test:"] # [doc = ""] # [doc = " ```rust,compile_fail"] # [doc = " assert!(cargo_miri_test::make_true() == 5);"] # [doc = " ```"] # [doc = ""] # [doc = " Post-monomorphization error in `compile_fail` test:"] # [doc = ""] # [doc = " ```rust,compile_fail"] # [doc = " struct Fail<T>(T);"] # [doc = " impl<T> Fail<T> {"] # [doc = "     const C: () = panic!();"] # [doc = " }"] # [doc = ""] # [doc = " let _val = Fail::<i32>::C;"] # [doc = " ```"] # [unsafe (no_mangle)] pub fn make_true () -> bool { proc_macro_crate :: use_the_dependency ! () ; issue_1567 :: use_the_dependency () ; issue_1705 :: use_the_dependency () ; issue_1691 :: use_me () }
};
}
