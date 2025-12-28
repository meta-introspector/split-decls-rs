macro_rules! deps {
    () => {
        TransmuteFrom!();
        MaybeUninit!();
        Valid!();
        Uninit!();
    };
}

macro_rules! impl_390 {
    () => {
        deps!();
        unsafe impl < T > TransmuteFrom < T , Uninit , Valid > for MaybeUninit < T > { }
    };
}

impl_390!();