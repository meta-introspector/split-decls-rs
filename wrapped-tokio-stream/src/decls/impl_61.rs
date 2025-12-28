macro_rules! deps {
    () => {
        Pending!();
    };
}

macro_rules! impl_61 {
    () => {
        deps!();
        impl < T > Unpin for Pending < T > { }
    };
}

impl_61!();