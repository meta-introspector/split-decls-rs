macro_rules! deps {
    () => {
        Unique!();
    };
}

macro_rules! UniqueBools {
    () => {
        deps!();
        type UniqueBools = Unique < bool > ;
    };
}

UniqueBools!();