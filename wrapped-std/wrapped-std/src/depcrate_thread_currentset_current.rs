// Generated macro for set_current (function)
macro_rules! Depcrate_thread_currentset_current {
() => {
// Module: crate::thread::current
// Provides: {"set_current"}
// Dependencies: {}
# [doc = " Tries to set the thread handle for the current thread. Fails if a handle was"] # [doc = " already set or if the thread ID of `thread` would change an already-set ID."] pub (super) fn set_current (thread : Thread) -> Result < () , Thread > { if CURRENT . get () != NONE { return Err (thread) ; } match id :: get () { Some (id) if id == thread . id () => { } None => id :: set (thread . id ()) , _ => return Err (thread) , } crate :: sys :: thread_local :: guard :: enable () ; CURRENT . set (thread . into_raw () . cast_mut ()) ; Ok (()) }
};
}
