macro_rules! RandomState {
    () => {
        # [cfg (all (feature = "preserve_order" , feature = "fast_hash"))] type RandomState = foldhash :: fast :: RandomState ;
    };
}

RandomState!()