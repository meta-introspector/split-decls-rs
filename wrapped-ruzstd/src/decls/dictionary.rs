macro_rules! dictionary {
    () => {
        # [cfg (feature = "dict_builder")] # [cfg_attr (docsrs , doc (cfg (feature = "dict_builder")))] pub mod dictionary ;
    };
}

dictionary!();