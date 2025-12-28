macro_rules! deps {
    () => {
        Tid!();
        Config!();
    };
}

macro_rules! impl_173 {
    () => {
        deps!();
        impl < C : cfg :: Config > Copy for Tid < C > { }
    };
}

impl_173!();