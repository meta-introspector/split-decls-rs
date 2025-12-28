macro_rules! rand {
    () => {
        # [cfg (not (windows))] # [cfg (feature = "rand")] # [cfg_attr (docsrs , doc (cfg (feature = "rand")))] pub mod rand ;
    };
}

rand!();