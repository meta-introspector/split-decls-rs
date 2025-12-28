macro_rules! deps {
    () => {
        DynSend!();
    };
}

macro_rules! impl_264 {
    () => {
        deps!();
        # [cfg (any (unix , target_os = "hermit" , all (target_vendor = "fortanix" , target_env = "sgx") , target_os = "solid_asp3" , target_os = "wasi" , target_os = "xous"))] impl ! DynSend for std :: env :: VarsOs { }
    };
}

impl_264!();