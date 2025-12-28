macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! prctl_1arg {
    () => {
        deps!();
        # [inline] pub (crate) unsafe fn prctl_1arg (option : c_int) -> io :: Result < c_int > { const NULL : * mut c_void = null_mut () ; syscalls :: prctl (option , NULL , NULL , NULL , NULL) }
    };
}

prctl_1arg!();