macro_rules! deps {
    () => {
        SendSyncPhantomData!();
    };
}

macro_rules! impl_69 {
    () => {
        deps!();
        impl < T : ? Sized > PartialEq for SendSyncPhantomData < T > { fn eq (& self , _other : & Self) -> bool { true } }
    };
}

impl_69!()