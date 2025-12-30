// Generated macro for dictionary_insert_value_retain_release (function)
macro_rules! Depcrate_test_foundation_retain_semanticsdictionary_insert_value_retain_release {
() => {
// Module: crate::test_foundation_retain_semantics
// Provides: {"dictionary_insert_value_retain_release"}
// Dependencies: {}
# [test] fn dictionary_insert_value_retain_release () { let dict = NSMutableDictionary :: new () ; dict . insert (& * NSNumber :: new_i32 (1) , & * RcTestObject :: new ()) ; let to_insert = RcTestObject :: new () ; let mut expected = ThreadTestData :: current () ; dict . insert (& * NSNumber :: new_i32 (1) , & to_insert) ; expected . retain += 1 ; expected . release += 1 ; expected . drop += 1 ; expected . assert_current () ; }
};
}
