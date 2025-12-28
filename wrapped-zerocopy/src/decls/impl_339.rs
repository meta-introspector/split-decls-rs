macro_rules! deps {
    () => {
        Uninit!();
        CastableFrom!();
    };
}

macro_rules! impl_339 {
    () => {
        deps!();
        unsafe impl < ST : ? Sized , DT : ? Sized > CastableFrom < ST , Uninit , Uninit > for DT { }
    };
}

impl_339!()