// Generated macro for adjust_limit (function)
macro_rules! Depcrate_os_xous_ffiadjust_limit {
() => {
// Module: crate::os::xous::ffi
// Provides: {"adjust_limit"}
// Dependencies: {}
# [doc = " Adjusts the given `knob` limit to match the new value `new`. The current value must"] # [doc = " match the `current` in order for this to take effect."] # [doc = ""] # [doc = " The new value is returned as a result of this call. If the call fails, then the old"] # [doc = " value is returned. In either case, this function returns successfully."] # [doc = ""] # [doc = " An error is generated if the `knob` is not a valid limit, or if the call"] # [doc = " would not succeed."] pub (crate) fn adjust_limit (knob : Limits , current : usize , new : usize) -> Result < usize , Error > { let mut a0 = Syscall :: AdjustProcessLimit as usize ; let mut a1 = knob as usize ; let a2 = current ; let a3 = new ; let a4 = 0 ; let a5 = 0 ; let a6 = 0 ; let a7 = 0 ; unsafe { core :: arch :: asm ! ("ecall" , inlateout ("a0") a0 , inlateout ("a1") a1 , inlateout ("a2") a2 => _ , inlateout ("a3") a3 => _ , inlateout ("a4") a4 => _ , inlateout ("a5") a5 => _ , inlateout ("a6") a6 => _ , inlateout ("a7") a7 => _ ,) } ; let result = a0 ; if result == SyscallResult :: Scalar2 as usize && a1 == knob as usize { Ok (a2) } else if result == SyscallResult :: Scalar5 as usize && a1 == knob as usize { Ok (a1) } else if result == SyscallResult :: Error as usize { Err (a1 . into ()) } else { Err (Error :: InternalError) } }
};
}
