macro_rules! origin {
    () => {
        # [cfg (feature = "extended-siginfo")] # [cfg_attr (docsrs , doc (cfg (feature = "extended-siginfo")))] pub mod origin ;
    };
}

origin!()