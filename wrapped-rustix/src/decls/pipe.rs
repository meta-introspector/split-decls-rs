macro_rules! pipe {
    () => {
        # [cfg (feature = "pipe")] # [cfg_attr (docsrs , doc (cfg (feature = "pipe")))] # [cfg (not (any (windows , target_os = "wasi")))] pub mod pipe ;
    };
}

pipe!();