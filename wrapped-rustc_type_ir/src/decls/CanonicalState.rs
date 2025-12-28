macro_rules! deps {
    () => {
        Canonical!();
        State!();
    };
}

macro_rules! CanonicalState {
    () => {
        deps!();
        pub type CanonicalState < I , T > = Canonical < I , State < I , T > > ;
    };
}

CanonicalState!()