macro_rules! deps {
    () => {
        Registry!();
    };
}

macro_rules! Terminator {
    () => {
        deps!();
        struct Terminator < 'a > (& 'a Arc < Registry >) ;
    };
}

Terminator!()