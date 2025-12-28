macro_rules! deps {
    () => {
        Values!();
    };
}

macro_rules! ValuesImpl {
    () => {
        deps!();
        # [cfg (feature = "preserve_order")] type ValuesImpl < 'a , K , V > = indexmap :: map :: Values < 'a , K , V > ;
    };
}

ValuesImpl!();