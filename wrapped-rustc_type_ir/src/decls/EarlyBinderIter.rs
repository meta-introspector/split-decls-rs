macro_rules! EarlyBinderIter {
    () => {
        pub struct EarlyBinderIter < I , T > { t : T , _tcx : PhantomData < I > , }
    };
}

EarlyBinderIter!();