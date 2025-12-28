macro_rules! deps {
    () => {
        SortPairs!();
    };
}

macro_rules! SortStrings {
    () => {
        deps!();
        type SortStrings = SortPairs < String > ;
    };
}

SortStrings!()