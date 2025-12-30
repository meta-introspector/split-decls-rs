// Generated macro for thread_init (function)
macro_rules! Depcrate_thread_atomics_spawnthread_init {
() => {
// Module: crate::thread::atomics::spawn
// Provides: {"thread_init"}
// Dependencies: {}
# [doc = " Common functionality between thread spawning initialization, regardless if a"] # [doc = " message is passed or not."] fn thread_init (name : Option < String > , scope : Option < & ScopeData >) -> Thread { let thread = Thread :: new_with_name (name) ; if let Some (scope) = & scope { scope . threads . fetch_add (1 , Ordering :: Relaxed) ; } thread }
};
}
