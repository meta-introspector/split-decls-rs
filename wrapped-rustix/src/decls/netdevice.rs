macro_rules! netdevice {
    () => {
        # [cfg (linux_kernel)] pub mod netdevice ;
    };
}

netdevice!();