macro_rules! deps {
    () => {
        Scan!();
    };
}

macro_rules! impl_284 {
    () => {
        deps!();
        # [cfg (feature = "full")] impl Copy for Scan { }
    };
}

impl_284!();