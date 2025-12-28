macro_rules! deps {
    () => {
        Borrowed!();
        TypeSize!();
        SizableArc!();
    };
}

macro_rules! impl_77 {
    () => {
        deps!();
        impl < T > TypeSize for SizableArc < T , Borrowed > { }
    };
}

impl_77!()