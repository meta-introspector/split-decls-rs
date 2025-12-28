macro_rules! deps {
    () => {
        Empty!();
    };
}

macro_rules! impl_44 {
    () => {
        deps!();
        impl < T > Unpin for Empty < T > { }
    };
}

impl_44!();