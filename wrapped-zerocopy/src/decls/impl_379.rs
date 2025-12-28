macro_rules! deps {
    () => {
        TransmuteFrom!();
        Initialized!();
    };
}

macro_rules! impl_379 {
    () => {
        deps!();
        unsafe impl < Src , Dst > TransmuteFrom < Src , Initialized , Initialized > for Dst where Src : ? Sized , Dst : ? Sized , { }
    };
}

impl_379!();