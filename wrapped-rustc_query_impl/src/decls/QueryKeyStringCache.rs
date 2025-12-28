macro_rules! QueryKeyStringCache {
    () => {
        pub (crate) struct QueryKeyStringCache { def_id_cache : FxHashMap < DefId , StringId > , }
    };
}

QueryKeyStringCache!()