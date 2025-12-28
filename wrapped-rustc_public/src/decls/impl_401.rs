macro_rules! deps {
    () => {
        Ty!();
        TraitRef!();
        ExistentialTraitRef!();
    };
}

macro_rules! impl_401 {
    () => {
        deps!();
        impl ExistentialTraitRef { pub fn with_self_ty (& self , self_ty : Ty) -> TraitRef { TraitRef :: new (self . def_id , self_ty , & self . generic_args) } }
    };
}

impl_401!();