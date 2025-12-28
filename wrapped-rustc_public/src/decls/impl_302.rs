macro_rules! deps {
    () => {
        TyKind!();
        Ty!();
    };
}

macro_rules! impl_302 {
    () => {
        deps!();
        impl Ty { pub fn kind (& self) -> TyKind { with (| context | context . ty_kind (* self)) } }
    };
}

impl_302!()