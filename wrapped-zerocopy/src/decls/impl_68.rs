macro_rules! deps {
    () => {
        SendSyncPhantomData!();
    };
}

macro_rules! impl_68 {
    () => {
        deps!();
        impl < T : ? Sized > Default for SendSyncPhantomData < T > { fn default () -> SendSyncPhantomData < T > { SendSyncPhantomData (PhantomData) } }
    };
}

impl_68!()