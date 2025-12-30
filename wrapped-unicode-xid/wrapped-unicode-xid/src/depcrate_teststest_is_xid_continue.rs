// Generated macro for test_is_xid_continue (function)
macro_rules! Depcrate_teststest_is_xid_continue {
() => {
// Module: crate::tests
// Provides: {"test_is_xid_continue"}
// Dependencies: {}
# [test] fn test_is_xid_continue () { let chars = ['0' , '9' , 'A' , 'Z' , 'a' , 'z' , '_' , '\u{1000d}' , '\u{10026}'] ; for ch in & chars { assert ! (super :: UnicodeXID :: is_xid_continue (* ch) , "{}" , ch) ; } }
};
}
