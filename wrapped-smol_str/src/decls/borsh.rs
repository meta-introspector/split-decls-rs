macro_rules! borsh {
    () => {
        # [cfg (feature = "borsh")] # [cfg_attr (docsrs , doc (cfg (feature = "borsh")))] mod borsh ;
    };
}

borsh!()