macro_rules! cfg_unstable {
    () => {
        macro_rules ! cfg_unstable { ($ ($ item : item) *) => { $ (# [cfg (tokio_unstable)] # [cfg_attr (docsrs , doc (cfg (tokio_unstable)))] $ item) * } ; }
    };
}

cfg_unstable!()