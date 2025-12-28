macro_rules! libcap {
    () => {
        # [cfg (linux_kernel)] mod libcap ;
    };
}

libcap!();