macro_rules! deps {
    () => {
        TypeSize!();
        SizableRc!();
        Borrowed!();
    };
}

macro_rules! impl_84 {
    () => {
        deps!();
        impl < T > TypeSize for SizableRc < T , Borrowed > { }
    };
}

impl_84!();