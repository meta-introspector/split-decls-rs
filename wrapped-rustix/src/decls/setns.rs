macro_rules! setns {
    () => {
        # [cfg (linux_kernel)] mod setns ;
    };
}

setns!();