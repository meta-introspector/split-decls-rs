// Generated macro for connect_impl (function)
macro_rules! Depcrate_os_xous_fficonnect_impl {
() => {
// Module: crate::os::xous::ffi
// Provides: {"connect_impl"}
// Dependencies: {}
fn connect_impl (address : ServerAddress , blocking : bool) -> Result < Connection , Error > { let a0 = if blocking { Syscall :: Connect } else { Syscall :: TryConnect } as usize ; let address : [u32 ; 4] = address . into () ; let a1 : usize = address [0] . try_into () . unwrap () ; let a2 : usize = address [1] . try_into () . unwrap () ; let a3 : usize = address [2] . try_into () . unwrap () ; let a4 : usize = address [3] . try_into () . unwrap () ; let a5 = 0 ; let a6 = 0 ; let a7 = 0 ; let mut result : usize ; let mut value : usize ; unsafe { core :: arch :: asm ! ("ecall" , inlateout ("a0") a0 => result , inlateout ("a1") a1 => value , inlateout ("a2") a2 => _ , inlateout ("a3") a3 => _ , inlateout ("a4") a4 => _ , inlateout ("a5") a5 => _ , inlateout ("a6") a6 => _ , inlateout ("a7") a7 => _ ,) } ; if result == SyscallResult :: ConnectionId as usize { Ok (value . try_into () . unwrap ()) } else if result == SyscallResult :: Error as usize { Err (value . into ()) } else { Err (Error :: InternalError) } }
};
}
