macro_rules! deps {
    () => {
        Timespec!();
        ClockId!();
        Result!();
        Wait!();
    };
}

macro_rules! waitv {
    () => {
        deps!();
        # [doc = " `futex_waitv(waiters.as_ptr(), waiters.len(), flags, timeout, clockd)`—"] # [doc = " Wait on an array of futexes, wake on any."] # [doc = ""] # [doc = " This requires Linux ≥ 5.16."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [Linux]: https://www.kernel.org/doc/html/latest/userspace-api/futex2.html"] # [inline] pub fn waitv (waiters : & [Wait] , flags : WaitvFlags , timeout : Option < & Timespec > , clockid : ClockId ,) -> io :: Result < usize > { backend :: thread :: syscalls :: futex_waitv (waiters , flags , timeout , clockid) }
    };
}

waitv!();