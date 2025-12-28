macro_rules! deps {
    () => {
        DynSync!();
    };
}

macro_rules! impl_271 {
    () => {
        deps!();
        # [cfg (any (unix , target_os = "hermit" , all (target_vendor = "fortanix" , target_env = "sgx") , target_os = "solid_asp3" , target_os = "wasi" , target_os = "xous"))] impl ! DynSync for std :: env :: VarsOs { }
    };
}

impl_271!();