macro_rules! deps {
    () => {
        Aliasing!();
        BecauseImmutable!();
        Immutable!();
        Read!();
    };
}

macro_rules! impl_342 {
    () => {
        deps!();
        impl < A : Aliasing , T : ? Sized + crate :: Immutable > Read < A , BecauseImmutable > for T { }
    };
}

impl_342!()