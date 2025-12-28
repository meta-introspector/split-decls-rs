macro_rules! system {
    () => {
        # [cfg (feature = "system")] # [cfg (not (any (windows , target_os = "wasi")))] # [cfg_attr (docsrs , doc (cfg (feature = "system")))] pub mod system ;
    };
}

system!();