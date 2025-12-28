macro_rules! deps {
    () => {
        Cursor!();
    };
}

macro_rules! impl_97 {
    () => {
        deps!();
        impl < 'a > Eq for Cursor < 'a > { }
    };
}

impl_97!();