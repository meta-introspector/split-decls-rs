macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! dup2_stdout {
    () => {
        deps!();
        # [doc = " Utility function to safely `dup2` over stdout (fd 1)."] # [cfg (not (any (windows , target_os = "wasi")))] # [inline] pub fn dup2_stdout < Fd : AsFd > (fd : Fd) -> io :: Result < () > { let fd = fd . as_fd () ; if fd . as_raw_fd () != c :: STDOUT_FILENO { let mut target = ManuallyDrop :: new (unsafe { take_stdout () }) ; backend :: io :: syscalls :: dup2 (fd , & mut target) ? ; } Ok (()) }
    };
}

dup2_stdout!();