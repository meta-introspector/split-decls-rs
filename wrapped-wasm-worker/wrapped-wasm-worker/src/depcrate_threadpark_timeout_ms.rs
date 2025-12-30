// Generated macro for park_timeout_ms (function)
macro_rules! Depcrate_threadpark_timeout_ms {
() => {
// Module: crate::thread
// Provides: {"park_timeout_ms"}
// Dependencies: {}
# [doc = " See [`std::thread::park_timeout_ms()`]."] # [doc = ""] # [doc = " # Notes"] # [doc = ""] # [doc = " Unlike [`std::thread::park_timeout_ms()`], when using the atomics target"] # [doc = " feature, this will not panic on the main thread, worklet or any other"] # [doc = " unsupported thread type. However, on supported thread types, this will"] # [doc = " function correctly even without the atomics target feature."] # [doc = ""] # [doc = " Keep in mind that this call will do nothing unless the calling thread"] # [doc = " supports blocking, see"] # [doc = " [`web::has_block_support()`](crate::web::has_block_support)."] # [deprecated (note = "replaced by `web_thread::park_timeout`")] pub fn park_timeout_ms (ms : u32) { park_timeout (Duration :: from_millis (ms . into ())) ; }
};
}
