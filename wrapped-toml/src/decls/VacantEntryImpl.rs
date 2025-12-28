macro_rules! deps {
    () => {
        VacantEntry!();
    };
}

macro_rules! VacantEntryImpl {
    () => {
        deps!();
        # [cfg (feature = "preserve_order")] type VacantEntryImpl < 'a , K , V > = indexmap :: map :: VacantEntry < 'a , K , V > ;
    };
}

VacantEntryImpl!();