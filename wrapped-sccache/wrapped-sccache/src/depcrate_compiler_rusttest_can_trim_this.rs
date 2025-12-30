// Generated macro for test_can_trim_this (function)
macro_rules! Depcrate_compiler_rusttest_can_trim_this {
() => {
// Module: crate::compiler::rust
// Provides: {"test_can_trim_this"}
// Dependencies: {}
# [test] # [cfg (feature = "dist-client")] fn test_can_trim_this () { use crate :: test :: utils :: create_file ; let tempdir = tempfile :: Builder :: new () . prefix ("sccache_test") . tempdir () . unwrap () ; let tempdir = tempdir . path () ; let rlib_file = create_file (tempdir , "libtest.rlib" , | _f | Ok (())) . unwrap () ; assert ! (can_trim_this (& rlib_file)) ; let _ar_file = create_file (tempdir , "libtest.a" , | _f | Ok (())) . unwrap () ; assert ! (! can_trim_this (& rlib_file)) ; }
};
}
