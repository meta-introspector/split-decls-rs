// Generated macro for nested_out_of_line_mods_loaded (function)
macro_rules! Depcrate_test_mod_resolvernested_out_of_line_mods_loaded {
() => {
// Module: crate::test::mod_resolver
// Provides: {"nested_out_of_line_mods_loaded"}
// Dependencies: {}
# [test] fn nested_out_of_line_mods_loaded () { verify_mod_resolution ("tests/mod-resolver/issue-4874/main.rs" , & ["tests/mod-resolver/issue-4874/bar/baz.rs" , "tests/mod-resolver/issue-4874/foo/qux.rs" ,] ,) ; }
};
}
