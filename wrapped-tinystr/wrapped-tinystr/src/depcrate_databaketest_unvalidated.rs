// Generated macro for test_unvalidated (function)
macro_rules! Depcrate_databaketest_unvalidated {
() => {
// Module: crate::databake
// Provides: {"test_unvalidated"}
// Dependencies: {}
# [test] fn test_unvalidated () { test_bake ! (UnvalidatedTinyAsciiStr < 10 >, const , crate :: tinystr ! (10usize , "foo") . to_unvalidated () , tinystr) ; test_bake ! (UnvalidatedTinyAsciiStr < 3 >, const , crate :: UnvalidatedTinyAsciiStr :: from_utf8_unchecked (* b"AB\xCD") , tinystr) ; }
};
}
