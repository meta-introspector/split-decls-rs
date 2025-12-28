macro_rules! deps {
    () => {
        Config!();
        Generation!();
    };
}

macro_rules! impl_96 {
    () => {
        deps!();
        impl < C : cfg :: Config > Copy for Generation < C > { }
    };
}

impl_96!()