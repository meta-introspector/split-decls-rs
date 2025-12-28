macro_rules! deps {
    () => {
        Signal!();
        Pid!();
        Result!();
    };
}

macro_rules! tkill {
    () => {
        deps!();
        # [doc = " `tkill(tid, sig)`—Send a signal to a thread."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " Causing an individual thread to abruptly terminate without involving the"] # [doc = " process' thread runtime (such as the libpthread or the libc) evokes"] # [doc = " undefined behavior."] # [doc = ""] # [doc = " Also, this is not `tgkill`, so the warning about the hazard of recycled"] # [doc = " thread IDs applies."] # [doc = ""] # [doc = " There may be further safety hazards not yet documented here."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/tkill.2.html"] # [inline] pub unsafe fn tkill (tid : Pid , sig : Signal) -> io :: Result < () > { backend :: runtime :: syscalls :: tkill (tid , sig) }
    };
}

tkill!()