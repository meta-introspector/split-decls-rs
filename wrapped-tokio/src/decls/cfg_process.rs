macro_rules! cfg_process {
    () => {
        macro_rules ! cfg_process { ($ ($ item : item) *) => { $ (# [cfg (feature = "process")] # [cfg_attr (docsrs , doc (cfg (feature = "process")))] # [cfg (not (loom))] # [cfg (not (target_os = "wasi"))] $ item) * } }
    };
}

cfg_process!()