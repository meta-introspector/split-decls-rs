macro_rules! deps {
    () => {
        UpcastFrom!();
        Upcast!();
    };
}

macro_rules! impl_478 {
    () => {
        deps!();
        impl < I , T , U > Upcast < I , U > for T where U : UpcastFrom < I , T > , { fn upcast (self , interner : I) -> U { U :: upcast_from (self , interner) } }
    };
}

impl_478!()