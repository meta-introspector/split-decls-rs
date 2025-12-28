macro_rules! deps {
    () => {
        PaddingFree!();
    };
}

macro_rules! impl_28 {
    () => {
        deps!();
        impl < T : ? Sized > PaddingFree < T , 0 > for () { }
    };
}

impl_28!()