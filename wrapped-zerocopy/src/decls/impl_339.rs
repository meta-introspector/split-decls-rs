macro_rules! deps {
    () => {
        CastableFrom!();
        Uninit!();
    };
}

macro_rules! impl_339 {
    () => {
        deps!();
        unsafe impl < ST : ? Sized , DT : ? Sized > CastableFrom < ST , Uninit , Uninit > for DT { }
    };
}

impl_339!();