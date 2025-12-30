// Generated macro for test_is_not_xid_continue (function)
macro_rules! Depcrate_teststest_is_not_xid_continue {
() => {
// Module: crate::tests
// Provides: {"test_is_not_xid_continue"}
// Dependencies: {}
# [test] fn test_is_not_xid_continue () { let chars = ['\x00' , '\x01' , ' ' , '[' , '<' , '{' , '(' , '\u{02c2}' , '\u{ffff}' ,] ; for & ch in & chars { assert ! (! super :: UnicodeXID :: is_xid_continue (ch) , "{}" , ch) ; } }
};
}
