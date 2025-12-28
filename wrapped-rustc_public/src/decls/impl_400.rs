macro_rules! deps {
    () => {
        Binder!();
        TraitRef!();
        Ty!();
        ExistentialTraitRef!();
    };
}

macro_rules! impl_400 {
    () => {
        deps!();
        impl Binder < ExistentialTraitRef > { pub fn with_self_ty (& self , self_ty : Ty) -> Binder < TraitRef > { self . map_bound_ref (| trait_ref | trait_ref . with_self_ty (self_ty)) } }
    };
}

impl_400!();