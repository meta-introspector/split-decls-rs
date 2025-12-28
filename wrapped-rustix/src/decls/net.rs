macro_rules! net {
    () => {
        # [cfg (not (target_os = "wasi"))] # [cfg (feature = "net")] # [cfg_attr (docsrs , doc (cfg (feature = "net")))] pub mod net ;
    };
}

net!();