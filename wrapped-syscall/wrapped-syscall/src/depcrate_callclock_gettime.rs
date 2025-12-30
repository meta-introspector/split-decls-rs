// Generated macro for clock_gettime (function)
macro_rules! Depcrate_callclock_gettime {
() => {
// Module: crate::call
// Provides: {"clock_gettime"}
// Dependencies: {}
# [doc = " Get the current system time"] pub fn clock_gettime (clock : usize , tp : & mut TimeSpec) -> Result < usize > { unsafe { syscall2 (SYS_CLOCK_GETTIME , clock , tp as * mut TimeSpec as usize) } }
};
}
