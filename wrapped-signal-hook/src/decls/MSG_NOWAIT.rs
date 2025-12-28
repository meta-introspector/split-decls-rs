macro_rules! MSG_NOWAIT {
    () => {
        # [cfg (not (target_os = "aix"))] const MSG_NOWAIT : i32 = libc :: MSG_DONTWAIT ;
    };
}

MSG_NOWAIT!();