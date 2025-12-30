// Generated macro for stack_to_rc (function)
macro_rules! Depcrate_blockstack_to_rc {
() => {
// Module: crate::block
// Provides: {"stack_to_rc"}
// Dependencies: {}
# [test] fn stack_to_rc () { macro_rules ! test_with { ($ ctor : path) => { { let mut expected = Count :: current () ; let counter = CloneDropTracker :: new () ; expected . new += 1 ; expected . assert_current () ; # [allow (unused_unsafe)] let stack = unsafe { $ ctor (move || { let _ = & counter ; }) } ; expected . assert_current () ; let rc1 = stack . copy () ; expected . clone += 1 ; expected . assert_current () ; let rc2 = stack . copy () ; expected . clone += 1 ; expected . assert_current () ; let clone2 = rc2 . clone () ; expected . assert_current () ; drop (rc2) ; expected . assert_current () ; drop (stack) ; expected . drop += 1 ; expected . assert_current () ; drop (rc1) ; expected . drop += 1 ; expected . assert_current () ; drop (clone2) ; expected . drop += 1 ; expected . assert_current () ; } } ; } test_with ! (StackBlock :: new) ; test_with ! (StackBlock :: with_encoding ::< VoidToVoid >) ; }
};
}
