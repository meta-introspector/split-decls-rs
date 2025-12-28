macro_rules! deps {
    () => {
        Interner!();
        TypeError!();
    };
}

macro_rules! impl_35 {
    () => {
        deps!();
        impl < I : Interner > Eq for TypeError < I > { }
    };
}

impl_35!()