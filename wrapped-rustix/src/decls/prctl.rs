macro_rules! prctl {
    () => {
        # [cfg (any (feature = "process" , feature = "thread"))] # [cfg (linux_kernel)] mod prctl ;
    };
}

prctl!();