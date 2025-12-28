macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! set_name {
    () => {
        deps!();
        # [doc = " Set the name of the calling thread."] # [doc = ""] # [doc = " Unlike `pthread_setname_np`, this function silently truncates the name to"] # [doc = " 16 bytes, as the Linux syscall does."] # [doc = ""] # [doc = " # References"] # [doc = "  - [`prctl(PR_SET_NAME,…)`]"] # [doc = ""] # [doc = " [`prctl(PR_SET_NAME,…)`]: https://man7.org/linux/man-pages/man2/prctl.2.html"] # [inline] pub fn set_name (name : & CStr) -> io :: Result < () > { unsafe { prctl_2args (PR_SET_NAME , name . as_ptr () as * mut _) } . map (| _r | ()) }
    };
}

set_name!()