macro_rules! deserializer {
    () => {
        # [cfg (feature = "parse")] # [cfg (feature = "serde")] mod deserializer ;
    };
}

deserializer!();