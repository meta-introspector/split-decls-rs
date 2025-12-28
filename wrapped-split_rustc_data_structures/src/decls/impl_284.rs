macro_rules! deps {
    () => {
        DynSend!();
        FromDyn!();
    };
}

macro_rules! impl_284 {
    () => {
        deps!();
        unsafe impl < T : DynSend > Send for FromDyn < T > { }
    };
}

impl_284!();