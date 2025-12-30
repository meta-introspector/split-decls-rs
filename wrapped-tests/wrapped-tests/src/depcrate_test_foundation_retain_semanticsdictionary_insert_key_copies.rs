// Generated macro for dictionary_insert_key_copies (function)
macro_rules! Depcrate_test_foundation_retain_semanticsdictionary_insert_key_copies {
() => {
// Module: crate::test_foundation_retain_semantics
// Provides: {"dictionary_insert_key_copies"}
// Dependencies: {}
# [test] fn dictionary_insert_key_copies () { let dict = NSMutableDictionary :: new () ; let key1 = NSCopyingRcTestObject :: new () ; let mut expected = ThreadTestData :: current () ; dict . insert (& * key1 , & * NSNumber :: new_i32 (1)) ; expected . copy += 1 ; expected . alloc += 1 ; expected . init += 1 ; expected . assert_current () ; dict . removeAllObjects () ; expected . release += 1 ; expected . drop += 1 ; expected . assert_current () ; }
};
}
