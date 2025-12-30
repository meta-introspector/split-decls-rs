// Generated macro for blocking_scalar_impl (function)
macro_rules! Depcrate_os_xous_ffiblocking_scalar_impl {
() => {
// Module: crate::os::xous::ffi
// Provides: {"blocking_scalar_impl"}
// Dependencies: {}
fn blocking_scalar_impl (connection : Connection , args : [usize ; 5] , blocking : bool ,) -> Result < [usize ; 5] , Error > { let mut a0 = if blocking { Syscall :: SendMessage } else { Syscall :: TrySendMessage } as usize ; let mut a1 : usize = connection . try_into () . unwrap () ; let mut a2 = InvokeType :: BlockingScalar as usize ; let mut a3 = args [0] ; let mut a4 = args [1] ; let mut a5 = args [2] ; let a6 = args [3] ; let a7 = args [4] ; unsafe { core :: arch :: asm ! ("ecall" , inlateout ("a0") a0 , inlateout ("a1") a1 , inlateout ("a2") a2 , inlateout ("a3") a3 , inlateout ("a4") a4 , inlateout ("a5") a5 , inlateout ("a6") a6 => _ , inlateout ("a7") a7 => _ ,) } ; let result = a0 ; if result == SyscallResult :: Scalar1 as usize { Ok ([a1 , 0 , 0 , 0 , 0]) } else if result == SyscallResult :: Scalar2 as usize { Ok ([a1 , a2 , 0 , 0 , 0]) } else if result == SyscallResult :: Scalar5 as usize { Ok ([a1 , a2 , a3 , a4 , a5]) } else if result == SyscallResult :: Error as usize { Err (a1 . into ()) } else { Err (Error :: InternalError) } }
};
}
