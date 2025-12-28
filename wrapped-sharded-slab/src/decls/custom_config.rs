macro_rules! custom_config {
    () => {
        # [cfg (not (loom))] mod custom_config ;
    };
}

custom_config!()