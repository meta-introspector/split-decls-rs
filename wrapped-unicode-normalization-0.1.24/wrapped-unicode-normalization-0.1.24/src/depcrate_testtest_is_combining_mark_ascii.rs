// Generated macro for test_is_combining_mark_ascii (function)
macro_rules! Depcrate_testtest_is_combining_mark_ascii {
() => {
// Module: crate::test
// Provides: {"test_is_combining_mark_ascii"}
// Dependencies: {}
# [test] fn test_is_combining_mark_ascii () { for cp in 0 .. 0x7f { assert ! (! is_combining_mark (char :: from_u32 (cp) . unwrap ())) ; } }
};
}
