macro_rules! KEY_WILDCARD {
    () => {
        # [cfg (feature = "structured-data")] const KEY_WILDCARD : & str = "..." ;
    };
}

KEY_WILDCARD!()