macro_rules! deps {
    () => {
        UpcastFrom!();
    };
}

macro_rules! impl_480 {
    () => {
        deps!();
        impl < I , T > UpcastFrom < I , T > for T { fn upcast_from (from : T , _tcx : I) -> Self { from } }
    };
}

impl_480!()