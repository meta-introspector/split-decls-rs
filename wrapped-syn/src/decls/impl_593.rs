macro_rules! deps {
    () => {
        Precedence!();
    };
}

macro_rules! impl_593 {
    () => {
        deps!();
        impl Clone for Precedence { fn clone (& self) -> Self { * self } }
    };
}

impl_593!();