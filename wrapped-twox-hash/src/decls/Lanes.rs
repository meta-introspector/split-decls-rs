macro_rules! deps {
    () => {
        Lane!();
    };
}

macro_rules! Lanes {
    () => {
        deps!();
        type Lanes = [Lane ; 4] ;
    };
}

Lanes!();