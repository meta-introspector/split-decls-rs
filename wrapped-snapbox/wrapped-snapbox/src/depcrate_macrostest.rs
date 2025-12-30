// Generated macro for test (module)
macro_rules! Depcrate_macrostest {
() => {
// Module: crate::macros
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { # [test] fn direct_fn_path () { assert_eq ! (fn_path ! () , "snapbox::macros::test::direct_fn_path") ; } # [test] # [allow (clippy :: redundant_closure_call)] fn closure_fn_path () { (| | { assert_eq ! (fn_path ! () , "snapbox::macros::test::closure_fn_path") ; }) () ; } # [test] fn nested_fn_path () { fn nested () { assert_eq ! (fn_path ! () , "snapbox::macros::test::nested_fn_path::nested") ; } nested () ; } }
};
}
