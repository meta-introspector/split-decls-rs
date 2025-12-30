// Generated macro for stack_new_clone_drop (function)
macro_rules! Depcrate_blockstack_new_clone_drop {
() => {
// Module: crate::block
// Provides: {"stack_new_clone_drop"}
// Dependencies: {}
# [test] fn stack_new_clone_drop () { macro_rules ! test_with { ($ ctor : path) => { { let mut expected = Count :: current () ; let counter = CloneDropTracker :: new () ; expected . new += 1 ; expected . assert_current () ; # [allow (unused_unsafe)] let block = unsafe { $ ctor (move || { let _ = & counter ; }) } ; expected . assert_current () ; let clone = block . clone () ; expected . clone += 1 ; expected . assert_current () ; drop (clone) ; expected . drop += 1 ; expected . assert_current () ; drop (block) ; expected . drop += 1 ; expected . assert_current () ; } } ; } test_with ! (StackBlock :: new) ; test_with ! (StackBlock :: with_encoding ::< VoidToVoid >) ; }
};
}
