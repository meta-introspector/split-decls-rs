macro_rules! deps {
    () => {
        AsMap!();
    };
}

macro_rules! impl_4 {
    () => {
        deps!();
        impl AsMap for Record < '_ > { }
    };
}

impl_4!()