macro_rules! deps {
    () => {
        End!();
    };
}

macro_rules! impl_462 {
    () => {
        deps!();
        impl Clone for End { fn clone (& self) -> Self { * self } }
    };
}

impl_462!();