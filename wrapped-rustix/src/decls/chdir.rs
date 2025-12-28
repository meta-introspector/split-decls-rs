macro_rules! chdir {
    () => {
        # [cfg (not (target_os = "wasi"))] mod chdir ;
    };
}

chdir!();