// Generated macro for lend_mut_impl (function)
macro_rules! Depcrate_os_xous_ffilend_mut_impl {
() => {
// Module: crate::os::xous::ffi
// Provides: {"lend_mut_impl"}
// Dependencies: {}
fn lend_mut_impl (connection : Connection , opcode : usize , data : & mut [u8] , arg1 : usize , arg2 : usize , blocking : bool ,) -> Result < (usize , usize) , Error > { let mut a0 = if blocking { Syscall :: SendMessage } else { Syscall :: TrySendMessage } as usize ; let mut a1 : usize = connection . try_into () . unwrap () ; let mut a2 = InvokeType :: LendMut as usize ; let a3 = opcode ; let a4 = data . as_mut_ptr () as usize ; let a5 = data . len () ; let a6 = arg1 ; let a7 = arg2 ; unsafe { core :: arch :: asm ! ("ecall" , inlateout ("a0") a0 , inlateout ("a1") a1 , inlateout ("a2") a2 , inlateout ("a3") a3 => _ , inlateout ("a4") a4 => _ , inlateout ("a5") a5 => _ , inlateout ("a6") a6 => _ , inlateout ("a7") a7 => _ ,) } ; let result = a0 ; if result == SyscallResult :: MemoryReturned as usize { Ok ((a1 , a2)) } else if result == SyscallResult :: Error as usize { Err (a1 . into ()) } else { Err (Error :: InternalError) } }
};
}
