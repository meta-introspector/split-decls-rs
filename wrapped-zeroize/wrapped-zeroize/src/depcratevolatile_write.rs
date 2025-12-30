// Generated macro for volatile_write (function)
macro_rules! Depcratevolatile_write {
() => {
// Module: crate
// Provides: {"volatile_write"}
// Dependencies: {}
# [doc = " Perform a volatile write to the destination"] # [inline (always)] fn volatile_write < T : Copy + Sized > (dst : & mut T , src : T) { unsafe { ptr :: write_volatile (dst , src) } }
};
}
