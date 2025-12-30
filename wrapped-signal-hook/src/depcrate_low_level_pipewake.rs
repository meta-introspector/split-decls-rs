// Generated macro for wake (function)
macro_rules! Depcrate_low_level_pipewake {
() => {
// Module: crate::low_level::pipe
// Provides: {"wake"}
// Dependencies: {}
pub (crate) fn wake (pipe : RawFd , method : WakeMethod) { unsafe { let data = b"X" as * const _ as * const _ ; match method { WakeMethod :: Write => libc :: write (pipe , data , 1) , WakeMethod :: Send => libc :: send (pipe , data , 1 , MSG_NOWAIT) , } ; } }
};
}
