macro_rules! socketpair {
    () => {
        # [cfg (not (any (windows , target_os = "wasi")))] mod socketpair ;
    };
}

socketpair!();