macro_rules! id {
    () => {
        # [cfg (not (target_os = "wasi"))] mod id ;
    };
}

id!();