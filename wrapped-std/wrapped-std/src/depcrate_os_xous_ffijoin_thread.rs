// Generated macro for join_thread (function)
macro_rules! Depcrate_os_xous_ffijoin_thread {
() => {
// Module: crate::os::xous::ffi
// Provides: {"join_thread"}
// Dependencies: {}
# [doc = " Waits for the given thread to terminate and returns the exit code from that thread."] pub (crate) fn join_thread (thread_id : ThreadId) -> Result < usize , Error > { let mut a0 = Syscall :: JoinThread as usize ; let mut a1 = thread_id . into () ; let a2 = 0 ; let a3 = 0 ; let a4 = 0 ; let a5 = 0 ; let a6 = 0 ; let a7 = 0 ; unsafe { core :: arch :: asm ! ("ecall" , inlateout ("a0") a0 , inlateout ("a1") a1 , inlateout ("a2") a2 => _ , inlateout ("a3") a3 => _ , inlateout ("a4") a4 => _ , inlateout ("a5") a5 => _ , inlateout ("a6") a6 => _ , inlateout ("a7") a7 => _ ,) } ; let result = a0 ; if result == SyscallResult :: Scalar1 as usize { Ok (a1) } else if result == SyscallResult :: Scalar2 as usize { Ok (a1) } else if result == SyscallResult :: Scalar5 as usize { Ok (a1) } else if result == SyscallResult :: Error as usize { Err (a1 . into ()) } else { Err (Error :: InternalError) } }
};
}
