macro_rules! deps {
    () => {
        Ty!();
        TyKind!();
    };
}

macro_rules! impl_302 {
    () => {
        deps!();
        impl Ty { pub fn kind (& self) -> TyKind { with (| context | context . ty_kind (* self)) } }
    };
}

impl_302!();