macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! arm_set_tls {
    () => {
        deps!();
        # [cfg (target_arch = "arm")] # [inline] pub unsafe fn arm_set_tls (data : * mut c_void) -> io :: Result < () > { backend :: runtime :: syscalls :: tls :: arm_set_tls (data) }
    };
}

arm_set_tls!();