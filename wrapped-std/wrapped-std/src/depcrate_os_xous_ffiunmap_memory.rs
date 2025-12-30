// Generated macro for unmap_memory (function)
macro_rules! Depcrate_os_xous_ffiunmap_memory {
() => {
// Module: crate::os::xous::ffi
// Provides: {"unmap_memory"}
// Dependencies: {}
# [doc = " Destroys the given memory, returning it to the compiler."] # [doc = ""] # [doc = " Safety: The memory pointed to by `range` should not be used after this"] # [doc = " function returns, even if this function returns Err()."] pub (crate) unsafe fn unmap_memory < T > (range : * mut [T]) -> Result < () , Error > { let mut a0 = Syscall :: UnmapMemory as usize ; let mut a1 = range . as_mut_ptr () as usize ; let a2 = range . len () * size_of :: < T > () ; let a3 = 0 ; let a4 = 0 ; let a5 = 0 ; let a6 = 0 ; let a7 = 0 ; unsafe { core :: arch :: asm ! ("ecall" , inlateout ("a0") a0 , inlateout ("a1") a1 , inlateout ("a2") a2 => _ , inlateout ("a3") a3 => _ , inlateout ("a4") a4 => _ , inlateout ("a5") a5 => _ , inlateout ("a6") a6 => _ , inlateout ("a7") a7 => _ ,) } ; let result = a0 ; if result == SyscallResult :: Ok as usize { Ok (()) } else if result == SyscallResult :: Error as usize { Err (a1 . into ()) } else { Err (Error :: InternalError) } }
};
}
