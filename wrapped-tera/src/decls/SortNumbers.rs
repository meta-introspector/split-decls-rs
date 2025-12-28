macro_rules! deps {
    () => {
        OrderedF64!();
        SortPairs!();
    };
}

macro_rules! SortNumbers {
    () => {
        deps!();
        type SortNumbers = SortPairs < OrderedF64 > ;
    };
}

SortNumbers!()