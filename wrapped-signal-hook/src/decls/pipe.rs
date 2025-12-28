macro_rules! pipe {
    () => {
        # [cfg (not (windows))] # [cfg_attr (docsrs , doc (cfg (not (windows))))] pub mod pipe ;
    };
}

pipe!();