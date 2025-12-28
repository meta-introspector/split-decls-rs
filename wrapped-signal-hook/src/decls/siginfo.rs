macro_rules! siginfo {
    () => {
        # [cfg (feature = "extended-siginfo-raw")] # [cfg_attr (docsrs , doc (cfg (feature = "extended-siginfo-raw")))] pub mod siginfo ;
    };
}

siginfo!()