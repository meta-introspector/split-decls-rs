macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! prctl_3args {
    () => {
        deps!();
        # [inline] pub (crate) unsafe fn prctl_3args (option : c_int , arg2 : * mut c_void , arg3 : * mut c_void ,) -> io :: Result < c_int > { syscalls :: prctl (option , arg2 , arg3 , null_mut () , null_mut ()) }
    };
}

prctl_3args!()