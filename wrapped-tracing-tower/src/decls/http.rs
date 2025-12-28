macro_rules! http {
    () => {
        # [cfg (feature = "http")] # [cfg_attr (docsrs , doc (cfg (feature = "http")))] pub mod http ;
    };
}

http!()