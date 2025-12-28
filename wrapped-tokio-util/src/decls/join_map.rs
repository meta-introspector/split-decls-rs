macro_rules! join_map {
    () => {
        # [cfg (feature = "join-map")] mod join_map ;
    };
}

join_map!()