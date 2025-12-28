macro_rules! deps {
    () => {
        Initialized!();
        TransmuteFrom!();
    };
}

macro_rules! impl_379 {
    () => {
        deps!();
        unsafe impl < Src , Dst > TransmuteFrom < Src , Initialized , Initialized > for Dst where Src : ? Sized , Dst : ? Sized , { }
    };
}

impl_379!()