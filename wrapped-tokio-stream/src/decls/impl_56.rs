macro_rules! deps {
    () => {
        Once!();
    };
}

macro_rules! impl_56 {
    () => {
        deps!();
        impl < I > Unpin for Once < I > { }
    };
}

impl_56!();