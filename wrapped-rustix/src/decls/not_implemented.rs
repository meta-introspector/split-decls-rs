macro_rules! not_implemented {
    () => {
        # [cfg (doc)] # [cfg_attr (docsrs , doc (cfg (doc)))] pub mod not_implemented ;
    };
}

not_implemented!();