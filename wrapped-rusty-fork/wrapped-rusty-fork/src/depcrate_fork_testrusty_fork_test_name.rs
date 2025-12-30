// Generated macro for rusty_fork_test_name (macro)
macro_rules! Depcrate_fork_testrusty_fork_test_name {
() => {
// Module: crate::fork_test
// Provides: {"rusty_fork_test_name"}
// Dependencies: {}
# [doc = " Given the unqualified name of a `#[test]` function, produce a"] # [doc = " `&'static str` corresponding to the name of the test as filtered by the"] # [doc = " standard test harness."] # [doc = ""] # [doc = " This is internally used by `rusty_fork_test!` but is made available since"] # [doc = " other test wrapping implementations will likely need it too."] # [doc = ""] # [doc = " This does not currently produce a constant expression."] # [macro_export] macro_rules ! rusty_fork_test_name { ($ function_name : ident) => { $ crate :: fork_test :: fix_module_path (concat ! (module_path ! () , "::" , stringify ! ($ function_name))) } }
};
}
