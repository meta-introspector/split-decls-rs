macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! dup2_stdin {
    () => {
        deps!();
        # [doc = " Utility function to safely `dup2` over stdin (fd 0)."] # [cfg (not (any (windows , target_os = "wasi")))] # [inline] pub fn dup2_stdin < Fd : AsFd > (fd : Fd) -> io :: Result < () > { let fd = fd . as_fd () ; if fd . as_raw_fd () != c :: STDIN_FILENO { let mut target = ManuallyDrop :: new (unsafe { take_stdin () }) ; backend :: io :: syscalls :: dup2 (fd , & mut target) ? ; } Ok (()) }
    };
}

dup2_stdin!()