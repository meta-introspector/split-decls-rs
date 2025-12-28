macro_rules! deps {
    () => {
        Captures!();
    };
}

macro_rules! impl_129 {
    () => {
        deps!();
        impl < 'a , T : ? Sized > Captures < 'a > for T { }
    };
}

impl_129!();