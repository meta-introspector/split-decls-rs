macro_rules! deps {
    () => {
        SortPairs!();
    };
}

macro_rules! SortBools {
    () => {
        deps!();
        type SortBools = SortPairs < bool > ;
    };
}

SortBools!();