macro_rules! deps {
    () => {
        MinMaxIn!();
    };
}

macro_rules! MinMaxes {
    () => {
        deps!();
        struct MinMaxes (IndexVec < usize , MinMaxIn > , fn (usize) -> MinMaxIn) ;
    };
}

MinMaxes!();