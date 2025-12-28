macro_rules! chroot {
    () => {
        # [cfg (not (any (target_os = "fuchsia" , target_os = "wasi")))] mod chroot ;
    };
}

chroot!();