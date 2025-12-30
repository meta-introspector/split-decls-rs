// Generated macro for test_is_xid_start (function)
macro_rules! Depcrate_teststest_is_xid_start {
() => {
// Module: crate::tests
// Provides: {"test_is_xid_start"}
// Dependencies: {}
# [test] fn test_is_xid_start () { let chars = ['A' , 'Z' , 'a' , 'z' , '\u{1000d}' , '\u{10026}'] ; for ch in & chars { assert ! (super :: UnicodeXID :: is_xid_start (* ch) , "{}" , ch) ; } }
};
}
