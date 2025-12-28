macro_rules! deps {
    () => {
        DynSync!();
        FromDyn!();
    };
}

macro_rules! impl_285 {
    () => {
        deps!();
        unsafe impl < T : DynSync > Sync for FromDyn < T > { }
    };
}

impl_285!()