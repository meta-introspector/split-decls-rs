macro_rules! deps {
    () => {
        FromId!();
        Id!();
    };
}

macro_rules! impl_141 {
    () => {
        deps!();
        impl FromId for Id { # [inline] fn from_id (id : Id) -> Self { id } }
    };
}

impl_141!();