// Generated macro for retain_release_rc_block (function)
macro_rules! Depcrate_blockretain_release_rc_block {
() => {
// Module: crate::block
// Provides: {"retain_release_rc_block"}
// Dependencies: {}
# [test] fn retain_release_rc_block () { macro_rules ! test_with { ($ ctor : path) => { { let mut expected = Count :: current () ; let counter = CloneDropTracker :: new () ; expected . new += 1 ; expected . assert_current () ; # [allow (unused_unsafe)] let block = unsafe { $ ctor (move || { let _ = & counter ; }) } ; expected . assert_current () ; let ptr = &* block as * const Block < _ > as * mut AnyObject ; let obj = unsafe { Retained :: retain (ptr) } . unwrap () ; expected . assert_current () ; drop (block) ; expected . assert_current () ; drop (obj) ; expected . drop += 1 ; expected . assert_current () ; } } ; } test_with ! (RcBlock :: new) ; test_with ! (RcBlock :: with_encoding ::< _ , _ , _ , VoidToVoid >) ; }
};
}
