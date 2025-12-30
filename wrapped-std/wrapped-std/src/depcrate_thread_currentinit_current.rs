// Generated macro for init_current (function)
macro_rules! Depcrate_thread_currentinit_current {
() => {
// Module: crate::thread::current
// Provides: {"init_current"}
// Dependencies: {}
# [cold] fn init_current (current : * mut ()) -> Thread { if current == NONE { CURRENT . set (BUSY) ; let id = id :: get_or_init () ; let thread = Thread :: new (id , None) ; crate :: sys :: thread_local :: guard :: enable () ; CURRENT . set (thread . clone () . into_raw () . cast_mut ()) ; thread } else if current == BUSY { rtabort ! ("\n\
            Attempted to access thread-local data while allocating said data.\n\
            Do not access functions that allocate in the global allocator!\n\
            This is a bug in the global allocator.\n\
            ") } else { debug_assert_eq ! (current , DESTROYED) ; panic ! ("use of std::thread::current() is not possible after the thread's \
            local data has been destroyed") } }
};
}
