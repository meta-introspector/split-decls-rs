// Generated macro for scalar_impl (function)
macro_rules! Depcrate_os_xous_ffiscalar_impl {
() => {
// Module: crate::os::xous::ffi
// Provides: {"scalar_impl"}
// Dependencies: {}
fn scalar_impl (connection : Connection , args : [usize ; 5] , blocking : bool) -> Result < () , Error > { let mut a0 = if blocking { Syscall :: SendMessage } else { Syscall :: TrySendMessage } as usize ; let mut a1 : usize = connection . try_into () . unwrap () ; let a2 = InvokeType :: Scalar as usize ; let a3 = args [0] ; let a4 = args [1] ; let a5 = args [2] ; let a6 = args [3] ; let a7 = args [4] ; unsafe { core :: arch :: asm ! ("ecall" , inlateout ("a0") a0 , inlateout ("a1") a1 , inlateout ("a2") a2 => _ , inlateout ("a3") a3 => _ , inlateout ("a4") a4 => _ , inlateout ("a5") a5 => _ , inlateout ("a6") a6 => _ , inlateout ("a7") a7 => _ ,) } ; let result = a0 ; if result == SyscallResult :: Ok as usize { Ok (()) } else if result == SyscallResult :: Error as usize { Err (a1 . into ()) } else { Err (Error :: InternalError) } }
};
}
