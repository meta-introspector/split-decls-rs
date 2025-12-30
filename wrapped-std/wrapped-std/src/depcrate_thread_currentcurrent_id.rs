// Generated macro for current_id (function)
macro_rules! Depcrate_thread_currentcurrent_id {
() => {
// Module: crate::thread::current
// Provides: {"current_id"}
// Dependencies: {}
# [doc = " Gets the id of the thread that invokes it."] # [doc = ""] # [doc = " This function will always succeed, will always return the same value for"] # [doc = " one thread and is guaranteed not to call the global allocator."] # [inline] pub (crate) fn current_id () -> ThreadId { if ! id :: CHEAP { if let Some (id) = try_with_current (| t | t . map (| t | t . id ())) { return id ; } } id :: get_or_init () }
};
}
