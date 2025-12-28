macro_rules! deps {
    () => {
        Binder!();
        FnSig!();
    };
}

macro_rules! PolyFnSig {
    () => {
        deps!();
        pub type PolyFnSig = Binder < FnSig > ;
    };
}

PolyFnSig!()