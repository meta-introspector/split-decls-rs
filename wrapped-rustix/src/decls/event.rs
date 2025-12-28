macro_rules! event {
    () => {
        # [cfg (feature = "event")] # [cfg_attr (docsrs , doc (cfg (feature = "event")))] pub mod event ;
    };
}

event!()