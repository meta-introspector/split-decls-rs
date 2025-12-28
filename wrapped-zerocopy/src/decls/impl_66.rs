macro_rules! deps {
    () => {
        SendSyncPhantomData!();
    };
}

macro_rules! impl_66 {
    () => {
        deps!();
        unsafe impl < T : ? Sized > Send for SendSyncPhantomData < T > { }
    };
}

impl_66!();