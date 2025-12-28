macro_rules! deps {
    () => {
        RandomState!();
    };
}

macro_rules! MapImpl {
    () => {
        deps!();
        # [cfg (feature = "preserve_order")] type MapImpl < K , V > = IndexMap < K , V , RandomState > ;
    };
}

MapImpl!()