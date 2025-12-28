macro_rules! deps {
    () => {
        Result!();
        SecureComputingMode!();
    };
}

macro_rules! set_secure_computing_mode {
    () => {
        deps!();
        # [doc = " Set the secure computing mode for the calling thread, to limit the"] # [doc = " available system calls."] # [doc = ""] # [doc = " # References"] # [doc = "  - [`prctl(PR_SET_SECCOMP,…)`]"] # [doc = ""] # [doc = " [`prctl(PR_SET_SECCOMP,…)`]: https://man7.org/linux/man-pages/man2/prctl.2.html"] # [inline] pub fn set_secure_computing_mode (mode : SecureComputingMode) -> io :: Result < () > { unsafe { prctl_2args (PR_SET_SECCOMP , mode as usize as * mut _) } . map (| _r | ()) }
    };
}

set_secure_computing_mode!();