macro_rules! deps {
    () => {
        SizeError!();
    };
}

macro_rules! impl_200 {
    () => {
        deps!();
        impl < Src : Eq , Dst : ? Sized > Eq for SizeError < Src , Dst > { }
    };
}

impl_200!()