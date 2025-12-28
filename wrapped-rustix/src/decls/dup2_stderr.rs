macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! dup2_stderr {
    () => {
        deps!();
        # [doc = " Utility function to safely `dup2` over stderr (fd 2)."] # [cfg (not (any (windows , target_os = "wasi")))] # [inline] pub fn dup2_stderr < Fd : AsFd > (fd : Fd) -> io :: Result < () > { let fd = fd . as_fd () ; if fd . as_raw_fd () != c :: STDERR_FILENO { let mut target = ManuallyDrop :: new (unsafe { take_stderr () }) ; backend :: io :: syscalls :: dup2 (fd , & mut target) ? ; } Ok (()) }
    };
}

dup2_stderr!()