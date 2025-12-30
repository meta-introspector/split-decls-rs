// Generated macro for lang_start (function)
macro_rules! Depcrate_rtlang_start {
() => {
// Module: crate::rt
// Provides: {"lang_start"}
// Dependencies: {}
# [cfg (not (any (test , doctest)))] # [lang = "start"] fn lang_start < T : crate :: process :: Termination + 'static > (main : fn () -> T , argc : isize , argv : * const * const u8 , sigpipe : u8 ,) -> isize { lang_start_internal (& move | | crate :: sys :: backtrace :: __rust_begin_short_backtrace (main) . report () . to_i32 () , argc , argv , sigpipe ,) }
};
}
