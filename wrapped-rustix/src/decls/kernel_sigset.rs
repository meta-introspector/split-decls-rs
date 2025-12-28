macro_rules! kernel_sigset {
    () => {
        # [cfg (linux_kernel)] # [cfg (any (feature = "io_uring" , feature = "runtime"))] mod kernel_sigset ;
    };
}

kernel_sigset!()