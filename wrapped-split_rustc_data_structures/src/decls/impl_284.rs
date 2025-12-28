macro_rules! deps {
    () => {
        FromDyn!();
        DynSend!();
    };
}

macro_rules! impl_284 {
    () => {
        deps!();
        unsafe impl < T : DynSend > Send for FromDyn < T > { }
    };
}

impl_284!()