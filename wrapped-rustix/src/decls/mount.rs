macro_rules! mount {
    () => {
        # [cfg (linux_kernel)] # [cfg (feature = "mount")] # [cfg_attr (docsrs , doc (cfg (feature = "mount")))] pub mod mount ;
    };
}

mount!()