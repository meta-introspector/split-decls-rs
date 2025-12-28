macro_rules! deps {
    () => {
        Result!();
        ProcSelector!();
    };
}

macro_rules! procctl_set {
    () => {
        deps!();
        # [inline] pub (crate) unsafe fn procctl_set < P > (option : c_int , process : ProcSelector , data : & P ,) -> io :: Result < () > { procctl (option , process , (as_ptr (data) as * mut P) . cast ()) }
    };
}

procctl_set!();