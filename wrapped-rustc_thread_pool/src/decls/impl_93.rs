macro_rules! deps {
    () => {
        LatchRef!();
    };
}

macro_rules! impl_93 {
    () => {
        deps!();
        impl < L > LatchRef < '_ , L > { pub (super) fn new (inner : & L) -> LatchRef < '_ , L > { LatchRef { inner , marker : PhantomData } } }
    };
}

impl_93!()