// Generated macro for with_autorelease (function)
macro_rules! Depcrate_unix_apple_diskwith_autorelease {
() => {
// Module: crate::unix::apple::disk
// Provides: {"with_autorelease"}
// Dependencies: {}
# [doc = " Calls the provided closure in the context of a new autorelease pool that is drained"] # [doc = " before returning."] # [doc = ""] # [doc = " ## SAFETY:"] # [doc = " You must not return an Objective-C object that is autoreleased from this function since it"] # [doc = " will be freed before usable."] unsafe fn with_autorelease < T , F : FnOnce () -> T > (call : F) -> T { struct DrainPool { ctx : * mut c_void , } impl Drop for DrainPool { fn drop (& mut self) { unsafe { ffi :: objc_autoreleasePoolPop (self . ctx) } } } let _pool_ctx = DrainPool { ctx : unsafe { ffi :: objc_autoreleasePoolPush () } , } ; call () }
};
}
