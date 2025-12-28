macro_rules! deps {
    () => {
        Initialized!();
        FromBytes!();
        TransmuteFrom!();
        Valid!();
    };
}

macro_rules! impl_378 {
    () => {
        deps!();
        unsafe impl < Src , Dst > TransmuteFrom < Src , Initialized , Valid > for Dst where Src : ? Sized , Dst : FromBytes + ? Sized , { }
    };
}

impl_378!();