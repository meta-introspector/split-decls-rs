macro_rules! deps {
    () => {
        MaxReached!();
    };
}

macro_rules! Maxes {
    () => {
        deps!();
        struct Maxes (IndexVec < usize , MaxReached > , fn (usize) -> usize) ;
    };
}

Maxes!()