macro_rules! deps {
    () => {
        Interner!();
        EarlyBinder!();
    };
}

macro_rules! impl_237 {
    () => {
        deps!();
        impl < I : Interner , T > EarlyBinder < I , Option < T > > { pub fn transpose (self) -> Option < EarlyBinder < I , T > > { self . value . map (| value | EarlyBinder { value , _tcx : PhantomData }) } }
    };
}

impl_237!();