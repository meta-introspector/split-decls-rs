// Generated macro for test_is_not_xid_start (function)
macro_rules! Depcrate_teststest_is_not_xid_start {
() => {
// Module: crate::tests
// Provides: {"test_is_not_xid_start"}
// Dependencies: {}
# [test] fn test_is_not_xid_start () { let chars = ['\x00' , '\x01' , '0' , '9' , ' ' , '[' , '<' , '{' , '(' , '\u{02c2}' , '\u{ffff}' ,] ; for ch in & chars { assert ! (! super :: UnicodeXID :: is_xid_start (* ch) , "{}" , ch) ; } }
};
}
