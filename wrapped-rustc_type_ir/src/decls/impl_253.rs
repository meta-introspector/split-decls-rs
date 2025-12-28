macro_rules! deps {
    () => {
        EarlyBinderIter!();
        EarlyBinder!();
        Interner!();
    };
}

macro_rules! impl_253 {
    () => {
        deps!();
        impl < I : Interner , T : IntoIterator > EarlyBinder < I , T > { pub fn transpose_iter (self) -> EarlyBinderIter < I , T :: IntoIter > { EarlyBinderIter { t : self . value . into_iter () , _tcx : PhantomData } } }
    };
}

impl_253!();