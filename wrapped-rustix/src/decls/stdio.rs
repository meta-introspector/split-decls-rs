macro_rules! stdio {
    () => {
        # [cfg (not (windows))] # [cfg (feature = "stdio")] # [cfg_attr (docsrs , doc (cfg (feature = "stdio")))] pub mod stdio ;
    };
}

stdio!();