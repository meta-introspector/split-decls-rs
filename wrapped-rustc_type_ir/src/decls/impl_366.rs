macro_rules! deps {
    () => {
        TraitRef!();
        ExistentialTraitRef!();
        Interner!();
        Binder!();
        Ty!();
    };
}

macro_rules! impl_366 {
    () => {
        deps!();
        impl < I : Interner > ty :: Binder < I , ExistentialTraitRef < I > > { pub fn def_id (& self) -> I :: TraitId { self . skip_binder () . def_id } # [doc = " Object types don't have a self type specified. Therefore, when"] # [doc = " we convert the principal trait-ref into a normal trait-ref,"] # [doc = " you must give *some* self type. A common choice is `mk_err()`"] # [doc = " or some placeholder type."] pub fn with_self_ty (& self , cx : I , self_ty : I :: Ty) -> ty :: Binder < I , TraitRef < I > > { self . map_bound (| trait_ref | trait_ref . with_self_ty (cx , self_ty)) } }
    };
}

impl_366!()