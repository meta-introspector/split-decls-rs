macro_rules! deps {
    () => {
        TryInitError!();
    };
}

macro_rules! impl_149 {
    () => {
        deps!();
        impl TryInitError { # [cfg (feature = "std")] fn new (e : impl Into < Box < dyn Error + Send + Sync + 'static > >) -> Self { Self { inner : e . into () } } # [cfg (not (feature = "std"))] fn new < T > (_ : T) -> Self { Self { _p : () } } }
    };
}

impl_149!();