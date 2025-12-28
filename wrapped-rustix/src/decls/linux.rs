macro_rules! linux {
    () => {
        # [cfg (linux_kernel)] mod linux ;
    };
}

linux!()