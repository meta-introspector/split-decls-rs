macro_rules! deps {
    () => {
        Initialized!();
        CastableFrom!();
    };
}

macro_rules! impl_340 {
    () => {
        deps!();
        unsafe impl < ST : ? Sized , DT : ? Sized > CastableFrom < ST , Initialized , Initialized > for DT { }
    };
}

impl_340!()