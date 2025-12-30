// Generated macro for sendfd (function)
macro_rules! Depcrate_callsendfd {
() => {
// Module: crate::call
// Provides: {"sendfd"}
// Dependencies: {}
# [doc = " Send a file descriptor `fd`, handled by the scheme providing `receiver_socket`. `flags` is"] # [doc = " currently unused (must be zero), and `arg` is included in the scheme call."] # [doc = ""] # [doc = " The scheme can return an arbitrary value."] pub fn sendfd (receiver_socket : usize , fd : usize , flags : usize , arg : u64) -> Result < usize > { # [cfg (target_pointer_width = "32")] unsafe { syscall5 (SYS_SENDFD , receiver_socket , fd , flags , arg as u32 as usize , (arg >> 32) as u32 as usize ,) } # [cfg (target_pointer_width = "64")] unsafe { syscall4 (SYS_SENDFD , receiver_socket , fd , flags , arg as usize) } }
};
}
