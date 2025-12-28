macro_rules! deps {
    () => {
        SortPairs!();
        OrderedF64!();
    };
}

macro_rules! SortNumbers {
    () => {
        deps!();
        type SortNumbers = SortPairs < OrderedF64 > ;
    };
}

SortNumbers!();