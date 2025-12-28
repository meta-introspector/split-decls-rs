macro_rules! deps {
    () => {
        ArrayLen!();
        SortPairs!();
    };
}

macro_rules! SortArrays {
    () => {
        deps!();
        type SortArrays = SortPairs < ArrayLen > ;
    };
}

SortArrays!()