// Generated macro for dictionary_get_key_copies (function)
macro_rules! Depcrate_test_foundation_retain_semanticsdictionary_get_key_copies {
() => {
// Module: crate::test_foundation_retain_semantics
// Provides: {"dictionary_get_key_copies"}
// Dependencies: {}
# [test] fn dictionary_get_key_copies () { let dict = NSMutableDictionary :: new () ; let key1 = NSCopyingRcTestObject :: new () ; dict . insert (& * key1 , & * NSNumber :: new_i32 (1)) ; let expected = ThreadTestData :: current () ; let _ = dict . objectForKey (& key1) ; expected . assert_current () ; }
};
}
