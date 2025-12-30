// Generated macro for current_or_unnamed (function)
macro_rules! Depcrate_thread_currentcurrent_or_unnamed {
() => {
// Module: crate::thread::current
// Provides: {"current_or_unnamed"}
// Dependencies: {}
# [doc = " Gets a handle to the thread that invokes it. If the handle stored in thread-"] # [doc = " local storage was already destroyed, this creates a new unnamed temporary"] # [doc = " handle to allow thread parking in nearly all situations."] pub (crate) fn current_or_unnamed () -> Thread { let current = CURRENT . get () ; if current > DESTROYED { unsafe { let current = ManuallyDrop :: new (Thread :: from_raw (current)) ; (* current) . clone () } } else if current == DESTROYED { Thread :: new (id :: get_or_init () , None) } else { init_current (current) } }
};
}
