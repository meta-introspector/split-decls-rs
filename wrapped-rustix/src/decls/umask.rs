macro_rules! umask {
    () => {
        # [cfg (not (target_os = "wasi"))] mod umask ;
    };
}

umask!()