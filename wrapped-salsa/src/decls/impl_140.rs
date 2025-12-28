macro_rules! deps {
    () => {
        AsId!();
        Id!();
    };
}

macro_rules! impl_140 {
    () => {
        deps!();
        impl AsId for Id { # [inline] fn as_id (& self) -> Id { * self } }
    };
}

impl_140!();