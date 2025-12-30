// Generated macro for park_timeout (function)
macro_rules! Depcrate_threadpark_timeout {
() => {
// Module: crate::thread
// Provides: {"park_timeout"}
// Dependencies: {}
# [doc = " See [`std::thread::park_timeout()`]."] # [doc = ""] # [doc = " # Notes"] # [doc = ""] # [doc = " Unlike [`std::thread::park_timeout()`], when using the atomics target"] # [doc = " feature, this will not panic on the main thread, worklet or any other"] # [doc = " unsupported thread type. However, on supported thread types, this will"] # [doc = " function correctly even without the atomics target feature."] # [doc = ""] # [doc = " Keep in mind that this call will do nothing unless the calling thread"] # [doc = " supports blocking, see"] # [doc = " [`web::has_block_support()`](crate::web::has_block_support)."] pub fn park_timeout (dur : Duration) { if has_block_support () { Pin :: new (& current () . 0 . parker) . park_timeout (dur) ; } }
};
}
