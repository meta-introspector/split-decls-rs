macro_rules! deps {
    () => {
        Config!();
        Tid!();
    };
}

macro_rules! impl_173 {
    () => {
        deps!();
        impl < C : cfg :: Config > Copy for Tid < C > { }
    };
}

impl_173!()