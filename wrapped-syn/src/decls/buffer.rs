macro_rules! buffer {
    () => {
        # [cfg (feature = "parsing")] # [cfg_attr (docsrs , doc (cfg (feature = "parsing")))] pub mod buffer ;
    };
}

buffer!()