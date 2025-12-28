macro_rules! membarrier {
    () => {
        # [cfg (linux_kernel)] mod membarrier ;
    };
}

membarrier!();