// Generated macro for has_spawn_support (function)
macro_rules! Depcrate_thread_atomicshas_spawn_support {
() => {
// Module: crate::thread::atomics
// Provides: {"has_spawn_support"}
// Dependencies: {}
# [doc = " Implementation for [`crate::web::has_spawn_support()`]. Make sure to"] # [doc = " call at least once on the main thread!"] pub (super) fn has_spawn_support () -> bool { # [doc = " We spawn only from the main thread, so we cache the result to be able to"] # [doc = " call it from other threads but get the result of the main thread."] # [allow (clippy :: disallowed_methods , reason = "this will be called at least once from the main thread before being cached")] static HAS_SPAWN_SUPPORT : LazyLock < bool > = LazyLock :: new (| | { super :: has_shared_array_buffer_support () && { let global : GlobalExt = js_sys :: global () . unchecked_into () ; ! global . worker () . is_undefined () } }) ; * HAS_SPAWN_SUPPORT }
};
}
