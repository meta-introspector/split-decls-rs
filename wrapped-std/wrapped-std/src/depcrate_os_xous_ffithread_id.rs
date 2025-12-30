// Generated macro for thread_id (function)
macro_rules! Depcrate_os_xous_ffithread_id {
() => {
// Module: crate::os::xous::ffi
// Provides: {"thread_id"}
// Dependencies: {}
# [doc = " Gets the current thread's ID."] pub (crate) fn thread_id () -> Result < ThreadId , Error > { let mut a0 = Syscall :: GetThreadId as usize ; let mut a1 = 0 ; let a2 = 0 ; let a3 = 0 ; let a4 = 0 ; let a5 = 0 ; let a6 = 0 ; let a7 = 0 ; unsafe { core :: arch :: asm ! ("ecall" , inlateout ("a0") a0 , inlateout ("a1") a1 , inlateout ("a2") a2 => _ , inlateout ("a3") a3 => _ , inlateout ("a4") a4 => _ , inlateout ("a5") a5 => _ , inlateout ("a6") a6 => _ , inlateout ("a7") a7 => _ ,) } ; let result = a0 ; if result == SyscallResult :: ThreadId as usize { Ok (a1 . into ()) } else if result == SyscallResult :: Error as usize { Err (a1 . into ()) } else { Err (Error :: InternalError) } }
};
}
