// Generated macro for fallback_and_try_to_resolve_external_submod_relative_to_current_dir_path (function)
macro_rules! Depcrate_test_mod_resolverfallback_and_try_to_resolve_external_submod_relative_to_current_dir_path {
() => {
// Module: crate::test::mod_resolver
// Provides: {"fallback_and_try_to_resolve_external_submod_relative_to_current_dir_path"}
// Dependencies: {}
# [test] fn fallback_and_try_to_resolve_external_submod_relative_to_current_dir_path () { verify_mod_resolution ("tests/mod-resolver/issue-5198/lib.rs" , & ["tests/mod-resolver/issue-5198/a.rs" , "tests/mod-resolver/issue-5198/lib/b.rs" , "tests/mod-resolver/issue-5198/lib/c/mod.rs" , "tests/mod-resolver/issue-5198/lib/c/e.rs" , "tests/mod-resolver/issue-5198/lib/c/d/f.rs" , "tests/mod-resolver/issue-5198/lib/c/d/g/mod.rs" ,] ,) }
};
}
