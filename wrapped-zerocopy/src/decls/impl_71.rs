macro_rules! deps {
    () => {
        SendSyncPhantomData!();
    };
}

macro_rules! impl_71 {
    () => {
        deps!();
        impl < T : ? Sized > Clone for SendSyncPhantomData < T > { fn clone (& self) -> Self { SendSyncPhantomData (PhantomData) } }
    };
}

impl_71!()