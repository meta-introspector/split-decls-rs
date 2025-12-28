macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! set_auxiliary_vector {
    () => {
        deps!();
        # [doc = " Set a new auxiliary vector."] # [doc = ""] # [doc = " # References"] # [doc = "  - [`prctl(PR_SET_MM,PR_SET_MM_AUXV,…)`]"] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " Please ensure the conditions necessary to safely call this function, as"] # [doc = " detailed in the references above."] # [doc = ""] # [doc = " [`prctl(PR_SET_MM,PR_SET_MM_AUXV,…)`]: https://man7.org/linux/man-pages/man2/prctl.2.html"] # [inline] # [doc (alias = "PR_SET_MM")] # [doc (alias = "PR_SET_MM_AUXV")] pub unsafe fn set_auxiliary_vector (auxv : & [* const c_void]) -> io :: Result < () > { syscalls :: prctl (PR_SET_MM , PR_SET_MM_AUXV as * mut _ , auxv . as_ptr () as * mut _ , auxv . len () as * mut _ , null_mut () ,) . map (| _r | ()) }
    };
}

set_auxiliary_vector!();