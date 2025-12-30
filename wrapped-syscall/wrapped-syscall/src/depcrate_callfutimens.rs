// Generated macro for futimens (function)
macro_rules! Depcrate_callfutimens {
() => {
// Module: crate::call
// Provides: {"futimens"}
// Dependencies: {}
pub fn futimens (fd : usize , times : & [TimeSpec]) -> Result < usize > { unsafe { syscall3 (SYS_FUTIMENS , fd , times . as_ptr () as usize , times . len () * mem :: size_of :: < TimeSpec > () ,) } }
};
}
