macro_rules! deps {
    () => {
        Iter!();
    };
}

macro_rules! IterImpl {
    () => {
        deps!();
        # [cfg (feature = "preserve_order")] type IterImpl < 'a , K , V > = indexmap :: map :: Iter < 'a , K , V > ;
    };
}

IterImpl!();