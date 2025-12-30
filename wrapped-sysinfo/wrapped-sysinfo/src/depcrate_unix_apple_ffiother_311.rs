// Generated macro for other_311 (other)
macro_rules! Depcrate_unix_apple_ffiother_311 {
() => {
// Module: crate::unix::apple::ffi
// Provides: {"other_311"}
// Dependencies: {}
# [cfg (feature = "disk")] # [link (name = "objc" , kind = "dylib")] unsafe extern "C" { pub fn objc_autoreleasePoolPop (pool : * mut libc :: c_void) ; pub fn objc_autoreleasePoolPush () -> * mut libc :: c_void ; }
};
}
