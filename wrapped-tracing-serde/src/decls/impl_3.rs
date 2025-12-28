macro_rules! deps {
    () => {
        AsMap!();
    };
}

macro_rules! impl_3 {
    () => {
        deps!();
        impl AsMap for Attributes < '_ > { }
    };
}

impl_3!()