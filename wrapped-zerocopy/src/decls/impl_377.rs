macro_rules! deps {
    () => {
        Initialized!();
        IntoBytes!();
        Valid!();
        TransmuteFrom!();
    };
}

macro_rules! impl_377 {
    () => {
        deps!();
        unsafe impl < Src , Dst > TransmuteFrom < Src , Valid , Initialized > for Dst where Src : IntoBytes + ? Sized , Dst : ? Sized , { }
    };
}

impl_377!()