macro_rules! deps {
    () => {
        TryFromBytes!();
        ValidityError!();
    };
}

macro_rules! impl_209 {
    () => {
        deps!();
        impl < Src : Eq , Dst : ? Sized + TryFromBytes > Eq for ValidityError < Src , Dst > { }
    };
}

impl_209!();