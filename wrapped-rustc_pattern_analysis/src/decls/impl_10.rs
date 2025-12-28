macro_rules! deps {
    () => {
        Captures!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl < 'a , T : ? Sized > Captures < 'a > for T { }
    };
}

impl_10!()