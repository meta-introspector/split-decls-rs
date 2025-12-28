macro_rules! deps {
    () => {
        DatabaseDownCaster!();
    };
}

macro_rules! impl_429 {
    () => {
        deps!();
        impl < DbView : ? Sized > Clone for DatabaseDownCaster < DbView > { fn clone (& self) -> Self { * self } }
    };
}

impl_429!();