macro_rules! thread {
    () => {
        # [cfg (not (windows))] # [cfg (feature = "thread")] # [cfg_attr (docsrs , doc (cfg (feature = "thread")))] pub mod thread ;
    };
}

thread!();