macro_rules! xxhash64 {
    () => {
        # [cfg (feature = "xxhash64")] # [cfg_attr (docsrs , doc (cfg (feature = "xxhash64")))] pub mod xxhash64 ;
    };
}

xxhash64!();