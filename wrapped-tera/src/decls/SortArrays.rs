macro_rules! deps {
    () => {
        SortPairs!();
        ArrayLen!();
    };
}

macro_rules! SortArrays {
    () => {
        deps!();
        type SortArrays = SortPairs < ArrayLen > ;
    };
}

SortArrays!();