// Generated macro for has_shared_array_buffer_support (function)
macro_rules! Depcrate_threadhas_shared_array_buffer_support {
() => {
// Module: crate::thread
// Provides: {"has_shared_array_buffer_support"}
// Dependencies: {}
# [doc = " Returns if [`SharedArrayBuffer`][js_sys::SharedArrayBuffer] is supported."] fn has_shared_array_buffer_support () -> bool { thread_local ! { static HAS_SHARED_ARRAY_BUFFER_SUPPORT : bool = { let global : GlobalExt = js_sys :: global () . unchecked_into () ; CROSS_ORIGIN_ISOLATED . with (Option :: clone) . unwrap_or_else (|| ! global . shared_array_buffer () . is_undefined ()) } ; } HAS_SHARED_ARRAY_BUFFER_SUPPORT . with (bool :: clone) }
};
}
