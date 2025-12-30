// Generated macro for panic (function)
macro_rules! Depcratepanic {
() => {
// Module: crate
// Provides: {"panic"}
// Dependencies: {}
# [inline (never)] # [panic_handler] fn panic (info : & core :: panic :: PanicInfo < '_ >) -> ! { println ! ("{info}") ; unsafe { sim :: _exit (1) } }
};
}
