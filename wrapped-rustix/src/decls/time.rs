macro_rules! time {
    () => {
        # [cfg (not (any (windows , target_os = "espidf")))] # [cfg (feature = "time")] # [cfg_attr (docsrs , doc (cfg (feature = "time")))] pub mod time ;
    };
}

time!()