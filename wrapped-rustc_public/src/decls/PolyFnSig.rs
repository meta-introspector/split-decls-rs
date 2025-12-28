macro_rules! deps {
    () => {
        FnSig!();
        Binder!();
    };
}

macro_rules! PolyFnSig {
    () => {
        deps!();
        pub type PolyFnSig = Binder < FnSig > ;
    };
}

PolyFnSig!();