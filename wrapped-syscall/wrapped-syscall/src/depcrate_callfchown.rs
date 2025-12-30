// Generated macro for fchown (function)
macro_rules! Depcrate_callfchown {
() => {
// Module: crate::call
// Provides: {"fchown"}
// Dependencies: {}
# [doc = " Change file ownership"] pub fn fchown (fd : usize , uid : u32 , gid : u32) -> Result < usize > { unsafe { syscall3 (SYS_FCHOWN , fd , uid as usize , gid as usize) } }
};
}
