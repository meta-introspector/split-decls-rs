// Generated macro for i32_to_buffer_index (function)
macro_rules! Depcrate_thread_atomicsi32_to_buffer_index {
() => {
// Module: crate::thread::atomics
// Provides: {"i32_to_buffer_index"}
// Dependencies: {}
# [doc = " Converts a reference to a pointer to [`i32`] to an index into the internal"] # [doc = " [`ArrayBuffer`](js_sys::ArrayBuffer) usable by methods of [`Atomics`]."] fn i32_to_buffer_index (ptr : * const i32) -> u32 { # [allow (clippy :: as_conversions)] let index = ptr as u32 / 4 ; index }
};
}
