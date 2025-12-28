macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! kernel_sigaltstack {
    () => {
        deps!();
        # [doc = " `sigaltstack(new, old)`—Modify and/or query a signal stack."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The memory region described by `new` must readable and writable and larger"] # [doc = " than the platform minimum signal stack size, and must have a guard region"] # [doc = " that conforms to the platform conventions for stack guard regions. The"] # [doc = " flags in `new` must be valid. This function does not diagnose all the"] # [doc = " errors that libc `sigaltstack` functions are documented as diagnosing."] # [doc = ""] # [doc = " While the memory region pointed to by `new` is registered as a signal"] # [doc = " stack, it must remain readable and writable, and must not be mutated in"] # [doc = " any way other than by having a signal handler run in it, and must not be"] # [doc = " the referent of a Rust reference from outside the signal handler."] # [doc = ""] # [doc = " If code elsewhere in the program is depending on signal handlers being run"] # [doc = " on a particular stack, this could break that code's assumptions. And if the"] # [doc = " caller is depending on signal handlers being run on the stack specified in"] # [doc = " the call, its assumptions could be broken by code elsewhere in the program"] # [doc = " calling this function."] # [doc = ""] # [doc = " There are probably things out there that assume that all alternate signal"] # [doc = " stack registration goes through libc, and this does not go through libc."] # [doc = ""] # [doc = " There may be further safety hazards not yet documented here."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/sigaltstack.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/sigaltstack.2.html"] # [inline] pub unsafe fn kernel_sigaltstack (new : Option < Stack >) -> io :: Result < Stack > { backend :: runtime :: syscalls :: kernel_sigaltstack (new) }
    };
}

kernel_sigaltstack!();