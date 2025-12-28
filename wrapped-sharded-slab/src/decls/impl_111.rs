macro_rules! deps {
    () => {
        RefCount!();
        Config!();
    };
}

macro_rules! impl_111 {
    () => {
        deps!();
        impl < C : cfg :: Config > Copy for RefCount < C > { }
    };
}

impl_111!()