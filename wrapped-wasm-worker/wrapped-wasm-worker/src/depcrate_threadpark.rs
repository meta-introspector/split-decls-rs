// Generated macro for park (function)
macro_rules! Depcrate_threadpark {
() => {
// Module: crate::thread
// Provides: {"park"}
// Dependencies: {}
# [doc = " See [`std::thread::park()`]."] # [doc = ""] # [doc = " # Notes"] # [doc = ""] # [doc = " Unlike [`std::thread::park()`], when using the atomics target feature, this"] # [doc = " will not panic on the main thread, worklet or any other unsupported thread"] # [doc = " type. However, on supported thread types, this will function correctly even"] # [doc = " without the atomics target feature."] # [doc = ""] # [doc = " Keep in mind that this call will do nothing unless the calling thread"] # [doc = " supports blocking, see"] # [doc = " [`web::has_block_support()`](crate::web::has_block_support)."] pub fn park () { if has_block_support () { Pin :: new (& current () . 0 . parker) . park () ; } }
};
}
