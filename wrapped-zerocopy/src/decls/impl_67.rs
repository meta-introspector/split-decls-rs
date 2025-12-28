macro_rules! deps {
    () => {
        SendSyncPhantomData!();
    };
}

macro_rules! impl_67 {
    () => {
        deps!();
        unsafe impl < T : ? Sized > Sync for SendSyncPhantomData < T > { }
    };
}

impl_67!()