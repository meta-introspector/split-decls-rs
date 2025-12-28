macro_rules! deps {
    () => {
        IntoIter!();
    };
}

macro_rules! IntoIterImpl {
    () => {
        deps!();
        # [cfg (feature = "preserve_order")] type IntoIterImpl < K , V > = indexmap :: map :: IntoIter < K , V > ;
    };
}

IntoIterImpl!()