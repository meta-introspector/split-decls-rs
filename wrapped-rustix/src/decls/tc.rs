macro_rules! tc {
    () => {
        # [cfg (not (target_os = "wasi"))] mod tc ;
    };
}

tc!();