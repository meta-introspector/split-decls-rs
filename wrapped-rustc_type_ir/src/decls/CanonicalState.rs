macro_rules! deps {
    () => {
        State!();
        Canonical!();
    };
}

macro_rules! CanonicalState {
    () => {
        deps!();
        pub type CanonicalState < I , T > = Canonical < I , State < I , T > > ;
    };
}

CanonicalState!();