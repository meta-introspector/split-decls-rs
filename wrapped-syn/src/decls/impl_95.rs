macro_rules! deps {
    () => {
        Cursor!();
    };
}

macro_rules! impl_95 {
    () => {
        deps!();
        impl < 'a > Copy for Cursor < 'a > { }
    };
}

impl_95!();