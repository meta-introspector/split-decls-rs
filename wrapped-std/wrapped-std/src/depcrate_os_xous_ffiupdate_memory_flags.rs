// Generated macro for update_memory_flags (function)
macro_rules! Depcrate_os_xous_ffiupdate_memory_flags {
() => {
// Module: crate::os::xous::ffi
// Provides: {"update_memory_flags"}
// Dependencies: {}
# [doc = " Adjusts the memory flags for the given range."] # [doc = ""] # [doc = " This can be used to remove flags from a given region in order to harden"] # [doc = " memory access. Note that flags may only be removed and may never be added."] # [doc = ""] # [doc = " Safety: The memory pointed to by `range` may become inaccessible or have its"] # [doc = " mutability removed. It is up to the caller to ensure that the flags specified"] # [doc = " by `new_flags` are upheld, otherwise the program will crash."] pub (crate) unsafe fn update_memory_flags < T > (range : * mut [T] , new_flags : MemoryFlags ,) -> Result < () , Error > { let mut a0 = Syscall :: UpdateMemoryFlags as usize ; let mut a1 = range . as_mut_ptr () as usize ; let a2 = range . len () * size_of :: < T > () ; let a3 = new_flags . bits () ; let a4 = 0 ; let a5 = 0 ; let a6 = 0 ; let a7 = 0 ; unsafe { core :: arch :: asm ! ("ecall" , inlateout ("a0") a0 , inlateout ("a1") a1 , inlateout ("a2") a2 => _ , inlateout ("a3") a3 => _ , inlateout ("a4") a4 => _ , inlateout ("a5") a5 => _ , inlateout ("a6") a6 => _ , inlateout ("a7") a7 => _ ,) } ; let result = a0 ; if result == SyscallResult :: Ok as usize { Ok (()) } else if result == SyscallResult :: Error as usize { Err (a1 . into ()) } else { Err (Error :: InternalError) } }
};
}
