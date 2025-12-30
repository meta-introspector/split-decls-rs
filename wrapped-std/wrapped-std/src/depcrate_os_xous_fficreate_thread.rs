// Generated macro for create_thread (function)
macro_rules! Depcrate_os_xous_fficreate_thread {
() => {
// Module: crate::os::xous::ffi
// Provides: {"create_thread"}
// Dependencies: {}
# [doc = " Creates a thread with a given stack and up to four arguments."] pub (crate) fn create_thread (start : * mut usize , stack : * mut [u8] , arg0 : usize , arg1 : usize , arg2 : usize , arg3 : usize ,) -> Result < ThreadId , Error > { let mut a0 = Syscall :: CreateThread as usize ; let mut a1 = start as usize ; let a2 = stack . as_mut_ptr () as usize ; let a3 = stack . len () ; let a4 = arg0 ; let a5 = arg1 ; let a6 = arg2 ; let a7 = arg3 ; unsafe { core :: arch :: asm ! ("ecall" , inlateout ("a0") a0 , inlateout ("a1") a1 , inlateout ("a2") a2 => _ , inlateout ("a3") a3 => _ , inlateout ("a4") a4 => _ , inlateout ("a5") a5 => _ , inlateout ("a6") a6 => _ , inlateout ("a7") a7 => _ ,) } ; let result = a0 ; if result == SyscallResult :: ThreadId as usize { Ok (a1 . into ()) } else if result == SyscallResult :: Error as usize { Err (a1 . into ()) } else { Err (Error :: InternalError) } }
};
}
