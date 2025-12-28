macro_rules! deps {
    () => {
        EndianMode!();
        Result!();
    };
}

macro_rules! set_endian_mode {
    () => {
        deps!();
        # [doc = " Set the endianness of the calling process."] # [doc = ""] # [doc = " # References"] # [doc = "  - [`prctl(PR_SET_ENDIAN,…)`]"] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " Please ensure the conditions necessary to safely call this function, as"] # [doc = " detailed in the references above."] # [doc = ""] # [doc = " [`prctl(PR_SET_ENDIAN,…)`]: https://man7.org/linux/man-pages/man2/prctl.2.html"] # [inline] # [doc (alias = "PR_SET_ENDIAN")] pub unsafe fn set_endian_mode (mode : EndianMode) -> io :: Result < () > { prctl_2args (PR_SET_ENDIAN , mode as usize as * mut _) . map (| _r | ()) }
    };
}

set_endian_mode!();