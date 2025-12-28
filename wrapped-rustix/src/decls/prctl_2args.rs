macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! prctl_2args {
    () => {
        deps!();
        # [inline] pub (crate) unsafe fn prctl_2args (option : c_int , arg2 : * mut c_void) -> io :: Result < c_int > { const NULL : * mut c_void = null_mut () ; syscalls :: prctl (option , arg2 , NULL , NULL , NULL) }
    };
}

prctl_2args!();