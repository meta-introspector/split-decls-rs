macro_rules! deps {
    () => {
        Cursor!();
    };
}

macro_rules! impl_96 {
    () => {
        deps!();
        impl < 'a > Clone for Cursor < 'a > { fn clone (& self) -> Self { * self } }
    };
}

impl_96!();