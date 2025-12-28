macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! no_new_privs {
    () => {
        deps!();
        # [doc = " Get the value of the `no_new_privs` attribute for the calling thread."] # [doc = ""] # [doc = " # References"] # [doc = "  - [`prctl(PR_GET_NO_NEW_PRIVS,…)`]"] # [doc = ""] # [doc = " [`prctl(PR_GET_NO_NEW_PRIVS,…)`]: https://man7.org/linux/man-pages/man2/prctl.2.html"] # [inline] pub fn no_new_privs () -> io :: Result < bool > { unsafe { prctl_1arg (PR_GET_NO_NEW_PRIVS) } . map (| r | r != 0) }
    };
}

no_new_privs!();