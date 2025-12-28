macro_rules! deps {
    () => {
        Cancelled!();
    };
}

macro_rules! impl_38 {
    () => {
        deps!();
        impl std :: error :: Error for Cancelled { }
    };
}

impl_38!();