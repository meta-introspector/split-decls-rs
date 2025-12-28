macro_rules! deps {
    () => {
        Result!();
        PTracer!();
    };
}

macro_rules! set_ptracer {
    () => {
        deps!();
        # [doc = " Declare that the ptracer process can `ptrace` the calling process as if it"] # [doc = " were a direct process ancestor."] # [doc = ""] # [doc = " # References"] # [doc = "  - [`prctl(PR_SET_PTRACER,…)`]"] # [doc = ""] # [doc = " [`prctl(PR_SET_PTRACER,…)`]: https://man7.org/linux/man-pages/man2/prctl.2.html"] # [inline] # [doc (alias = "PR_SET_PTRACER")] pub fn set_ptracer (tracer : PTracer) -> io :: Result < () > { let pid = match tracer { PTracer :: None => null_mut () , PTracer :: Any => PR_SET_PTRACER_ANY as * mut _ , PTracer :: ProcessID (pid) => pid . as_raw_nonzero () . get () as usize as * mut _ , } ; unsafe { prctl_2args (PR_SET_PTRACER , pid) } . map (| _r | ()) }
    };
}

set_ptracer!()