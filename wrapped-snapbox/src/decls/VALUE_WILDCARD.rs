macro_rules! VALUE_WILDCARD {
    () => {
        # [cfg (feature = "structured-data")] const VALUE_WILDCARD : & str = "{...}" ;
    };
}

VALUE_WILDCARD!()