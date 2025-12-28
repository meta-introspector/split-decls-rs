macro_rules! deps {
    () => {
        IntoBytes!();
        Initialized!();
        TransmuteFrom!();
        Valid!();
    };
}

macro_rules! impl_377 {
    () => {
        deps!();
        unsafe impl < Src , Dst > TransmuteFrom < Src , Valid , Initialized > for Dst where Src : IntoBytes + ? Sized , Dst : ? Sized , { }
    };
}

impl_377!();