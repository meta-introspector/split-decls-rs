// Generated macro for dup2_stdin (function)
macro_rules! Depcrate_stdiodup2_stdin {
() => {
// Module: crate::stdio
// Provides: {"dup2_stdin"}
// Dependencies: {}
# [doc = " Utility function to safely `dup2` over stdin (fd 0)."] # [cfg (not (any (windows , target_os = "wasi")))] # [inline] pub fn dup2_stdin < Fd : AsFd > (fd : Fd) -> io :: Result < () > { let fd = fd . as_fd () ; if fd . as_raw_fd () != c :: STDIN_FILENO { let mut target = ManuallyDrop :: new (unsafe { take_stdin () }) ; backend :: io :: syscalls :: dup2 (fd , & mut target) ? ; } Ok (()) }
};
}
