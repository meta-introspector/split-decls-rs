macro_rules! deps {
    () => {
        Lanes!();
    };
}

macro_rules! Accumulators {
    () => {
        deps!();
        # [derive (Clone , PartialEq)] struct Accumulators (Lanes) ;
    };
}

Accumulators!()