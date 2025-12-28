macro_rules! FxHasher {
    () => {
        pub (crate) type FxHasher = std :: hash :: BuildHasherDefault < rustc_hash :: FxHasher > ;
    };
}

FxHasher!();