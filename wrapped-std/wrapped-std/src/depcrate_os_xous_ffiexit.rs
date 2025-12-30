// Generated macro for exit (function)
macro_rules! Depcrate_os_xous_ffiexit {
() => {
// Module: crate::os::xous::ffi
// Provides: {"exit"}
// Dependencies: {}
# [doc = " Terminates the current process and returns the specified code to the parent process."] pub (crate) fn exit (return_code : u32) -> ! { let a0 = Syscall :: TerminateProcess as usize ; let a1 = return_code as usize ; let a2 = 0 ; let a3 = 0 ; let a4 = 0 ; let a5 = 0 ; let a6 = 0 ; let a7 = 0 ; unsafe { core :: arch :: asm ! ("ecall" , in ("a0") a0 , in ("a1") a1 , in ("a2") a2 , in ("a3") a3 , in ("a4") a4 , in ("a5") a5 , in ("a6") a6 , in ("a7") a7 ,) } ; unreachable ! () ; }
};
}
