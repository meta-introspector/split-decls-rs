// Generated macro for lend_impl (function)
macro_rules! Depcrate_os_xous_ffilend_impl {
() => {
// Module: crate::os::xous::ffi
// Provides: {"lend_impl"}
// Dependencies: {}
fn lend_impl (connection : Connection , opcode : usize , data : & [u8] , arg1 : usize , arg2 : usize , blocking : bool ,) -> Result < (usize , usize) , Error > { let mut a0 = if blocking { Syscall :: SendMessage } else { Syscall :: TrySendMessage } as usize ; let a1 : usize = connection . try_into () . unwrap () ; let a2 = InvokeType :: Lend as usize ; let a3 = opcode ; let a4 = data . as_ptr () as usize ; let a5 = data . len () ; let a6 = arg1 ; let a7 = arg2 ; let mut ret1 ; let mut ret2 ; unsafe { core :: arch :: asm ! ("ecall" , inlateout ("a0") a0 , inlateout ("a1") a1 => ret1 , inlateout ("a2") a2 => ret2 , inlateout ("a3") a3 => _ , inlateout ("a4") a4 => _ , inlateout ("a5") a5 => _ , inlateout ("a6") a6 => _ , inlateout ("a7") a7 => _ ,) } ; let result = a0 ; if result == SyscallResult :: MemoryReturned as usize { Ok ((ret1 , ret2)) } else if result == SyscallResult :: Error as usize { Err (ret1 . into ()) } else { Err (Error :: InternalError) } }
};
}
