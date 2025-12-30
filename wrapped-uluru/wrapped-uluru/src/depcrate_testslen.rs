// Generated macro for len (function)
macro_rules! Depcrate_testslen {
() => {
// Module: crate::tests
// Provides: {"len"}
// Dependencies: {}
# [test] fn len () { let mut cache = TestCache :: default () ; cache . insert (1) ; assert_eq ! (cache . len () , 1) ; assert_eq ! (items (& mut cache) , [1]) ; }
};
}
