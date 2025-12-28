macro_rules! pty {
    () => {
        # [cfg (not (windows))] # [cfg (not (target_os = "wasi"))] # [cfg (feature = "pty")] # [cfg_attr (docsrs , doc (cfg (feature = "pty")))] pub mod pty ;
    };
}

pty!();