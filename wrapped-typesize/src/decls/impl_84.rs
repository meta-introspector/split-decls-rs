macro_rules! deps {
    () => {
        Borrowed!();
        TypeSize!();
        SizableRc!();
    };
}

macro_rules! impl_84 {
    () => {
        deps!();
        impl < T > TypeSize for SizableRc < T , Borrowed > { }
    };
}

impl_84!()