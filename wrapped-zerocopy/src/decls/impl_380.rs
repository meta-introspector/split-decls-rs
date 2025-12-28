macro_rules! deps {
    () => {
        Validity!();
        TransmuteFrom!();
        Uninit!();
    };
}

macro_rules! impl_380 {
    () => {
        deps!();
        unsafe impl < Src , Dst , V > TransmuteFrom < Src , V , Uninit > for Dst where Src : ? Sized , Dst : ? Sized , V : Validity , { }
    };
}

impl_380!()