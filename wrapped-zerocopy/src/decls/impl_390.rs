macro_rules! deps {
    () => {
        TransmuteFrom!();
        Uninit!();
        MaybeUninit!();
        Valid!();
    };
}

macro_rules! impl_390 {
    () => {
        deps!();
        unsafe impl < T > TransmuteFrom < T , Uninit , Valid > for MaybeUninit < T > { }
    };
}

impl_390!()