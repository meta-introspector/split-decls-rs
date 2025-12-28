macro_rules! deps {
    () => {
        Response!();
        Interner!();
    };
}

macro_rules! impl_191 {
    () => {
        deps!();
        impl < I : Interner > Eq for Response < I > { }
    };
}

impl_191!();