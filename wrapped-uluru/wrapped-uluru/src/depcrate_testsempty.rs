// Generated macro for empty (function)
macro_rules! Depcrate_testsempty {
() => {
// Module: crate::tests
// Provides: {"empty"}
// Dependencies: {}
# [test] fn empty () { let mut cache = TestCache :: new () ; assert_eq ! (cache . is_empty () , true) ; assert_eq ! (items (& mut cache) , []) ; cache . insert (1) ; assert_eq ! (cache . is_empty () , false) ; }
};
}
