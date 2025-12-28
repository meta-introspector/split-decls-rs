macro_rules! deps {
    () => {
        OccupiedEntry!();
    };
}

macro_rules! OccupiedEntryImpl {
    () => {
        deps!();
        # [cfg (feature = "preserve_order")] type OccupiedEntryImpl < 'a , K , V > = indexmap :: map :: OccupiedEntry < 'a , K , V > ;
    };
}

OccupiedEntryImpl!();