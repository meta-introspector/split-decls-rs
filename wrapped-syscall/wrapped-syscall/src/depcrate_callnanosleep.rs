// Generated macro for nanosleep (function)
macro_rules! Depcrate_callnanosleep {
() => {
// Module: crate::call
// Provides: {"nanosleep"}
// Dependencies: {}
# [doc = " Sleep for the time specified in `req`"] pub fn nanosleep (req : & TimeSpec , rem : & mut TimeSpec) -> Result < usize > { unsafe { syscall2 (SYS_NANOSLEEP , req as * const TimeSpec as usize , rem as * mut TimeSpec as usize ,) } }
};
}
