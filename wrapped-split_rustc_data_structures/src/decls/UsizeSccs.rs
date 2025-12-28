macro_rules! deps {
    () => {
        Sccs!();
    };
}

macro_rules! UsizeSccs {
    () => {
        deps!();
        type UsizeSccs = Sccs < usize , usize > ;
    };
}

UsizeSccs!()