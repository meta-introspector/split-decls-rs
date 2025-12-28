macro_rules! deps {
    () => {
        DynamicPaddingFree!();
    };
}

macro_rules! impl_30 {
    () => {
        deps!();
        impl < T : ? Sized > DynamicPaddingFree < T , false > for () { }
    };
}

impl_30!();