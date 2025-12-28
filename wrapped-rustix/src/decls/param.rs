macro_rules! param {
    () => {
        # [cfg (not (any (windows , target_os = "espidf")))] # [cfg (feature = "param")] # [cfg_attr (docsrs , doc (cfg (feature = "param")))] pub mod param ;
    };
}

param!()