macro_rules! deps {
    () => {
        Date!();
        Release!();
    };
}

macro_rules! Bound {
    () => {
        deps!();
        pub enum Bound { Nightly (Date) , Stable (Release) , }
    };
}

Bound!()