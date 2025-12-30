// Generated macro for empty (function)
macro_rules! Depcrate_enumeratorempty {
() => {
// Module: crate::enumerator
// Provides: {"empty"}
// Dependencies: {}
# [test] # [cfg_attr (not (feature = "catch-all") , ignore = "aborts the test")] # [should_panic = "NSInvalidArgumentException"] fn empty () { let enumerator = NSEnumerator :: < NSObject > :: new () ; assert_eq ! (enumerator . iter () . count () , 0) ; }
};
}
