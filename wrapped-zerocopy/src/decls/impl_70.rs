macro_rules! deps {
    () => {
        SendSyncPhantomData!();
    };
}

macro_rules! impl_70 {
    () => {
        deps!();
        impl < T : ? Sized > Eq for SendSyncPhantomData < T > { }
    };
}

impl_70!();