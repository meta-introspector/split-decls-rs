// Generated macro for retain_release_stack_block (function)
macro_rules! Depcrate_blockretain_release_stack_block {
() => {
// Module: crate::block
// Provides: {"retain_release_stack_block"}
// Dependencies: {}
# [doc = " Retaining/releasing stack blocks is kinda weird and unsupported."] # [doc = ""] # [doc = " As an example, the reference count is not increased for stack blocks on"] # [doc = " Apple's runtime, while on GNUStep, the `-retain` returns the new block,"] # [doc = " which is generally very unexpected behaviour."] # [test] fn retain_release_stack_block () { let mut expected = Count :: current () ; let counter = CloneDropTracker :: new () ; expected . new += 1 ; expected . assert_current () ; let block = StackBlock :: new (move | | { let _ = & counter ; }) ; expected . assert_current () ; let ptr = & * block as * const Block < _ > as * mut AnyObject ; let obj = if cfg ! (feature = "gnustep-1-7") { let ptr = unsafe { objc2 :: ffi :: objc_retain (ptr . cast ()) . cast () } ; unsafe { Retained :: from_raw (ptr) } . unwrap () } else { unsafe { Retained :: retain (ptr) } . unwrap () } ; if cfg ! (feature = "gnustep-1-7") { expected . clone += 1 ; } expected . assert_current () ; drop (obj) ; if cfg ! (feature = "gnustep-1-7") { expected . drop += 1 ; } expected . assert_current () ; drop (block) ; expected . drop += 1 ; expected . assert_current () ; }
};
}
