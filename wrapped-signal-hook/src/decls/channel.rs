macro_rules! channel {
    () => {
        # [cfg (feature = "channel")] # [cfg_attr (docsrs , doc (cfg (feature = "channel")))] pub mod channel ;
    };
}

channel!()