macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! set_no_new_privs {
    () => {
        deps!();
        # [doc = " Set the calling thread's `no_new_privs` attribute."] # [doc = ""] # [doc = " # References"] # [doc = "  - [`prctl(PR_SET_NO_NEW_PRIVS,…)`]"] # [doc = ""] # [doc = " [`prctl(PR_SET_NO_NEW_PRIVS,…)`]: https://man7.org/linux/man-pages/man2/prctl.2.html"] # [inline] pub fn set_no_new_privs (no_new_privs : bool) -> io :: Result < () > { unsafe { prctl_2args (PR_SET_NO_NEW_PRIVS , usize :: from (no_new_privs) as * mut _) } . map (| _r | ()) }
    };
}

set_no_new_privs!()