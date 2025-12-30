// Generated macro for do_yield (function)
macro_rules! Depcrate_os_xous_ffido_yield {
() => {
// Module: crate::os::xous::ffi
// Provides: {"do_yield"}
// Dependencies: {}
# [doc = " Suspends the current thread and allow another thread to run. This thread may"] # [doc = " continue executing again immediately if there are no other threads available"] # [doc = " to run on the system."] pub (crate) fn do_yield () { let a0 = Syscall :: Yield as usize ; let a1 = 0 ; let a2 = 0 ; let a3 = 0 ; let a4 = 0 ; let a5 = 0 ; let a6 = 0 ; let a7 = 0 ; unsafe { core :: arch :: asm ! ("ecall" , inlateout ("a0") a0 => _ , inlateout ("a1") a1 => _ , inlateout ("a2") a2 => _ , inlateout ("a3") a3 => _ , inlateout ("a4") a4 => _ , inlateout ("a5") a5 => _ , inlateout ("a6") a6 => _ , inlateout ("a7") a7 => _ ,) } ; }
};
}
