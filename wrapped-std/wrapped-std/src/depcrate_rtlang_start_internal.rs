// Generated macro for lang_start_internal (function)
macro_rules! Depcrate_rtlang_start_internal {
() => {
// Module: crate::rt
// Provides: {"lang_start_internal"}
// Dependencies: {}
# [cfg (not (test))] fn lang_start_internal (main : & (dyn Fn () -> i32 + Sync + crate :: panic :: RefUnwindSafe) , argc : isize , argv : * const * const u8 , sigpipe : u8 ,) -> isize { panic :: catch_unwind (move | | { unsafe { init (argc , argv , sigpipe) } ; let ret_code = panic :: catch_unwind (main) . unwrap_or_else (move | payload | { let payload = panic :: AssertUnwindSafe (payload) ; panic :: catch_unwind (move | | drop ({ payload } . 0)) . unwrap_or_else (move | e | { mem :: forget (e) ; rtabort ! ("drop of the panic payload panicked") ; }) ; 101 }) ; let ret_code = ret_code as isize ; cleanup () ; crate :: sys :: exit_guard :: unique_thread_exit () ; ret_code }) . unwrap_or_else (handle_rt_panic) }
};
}
