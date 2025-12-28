macro_rules! deps {
    () => {
        HostEffectPredicate!();
        Interner!();
        Ty!();
    };
}

macro_rules! impl_392 {
    () => {
        deps!();
        impl < I : Interner > HostEffectPredicate < I > { pub fn self_ty (self) -> I :: Ty { self . trait_ref . self_ty () } pub fn with_replaced_self_ty (self , interner : I , self_ty : I :: Ty) -> Self { Self { trait_ref : self . trait_ref . with_replaced_self_ty (interner , self_ty) , .. self } } pub fn def_id (self) -> I :: TraitId { self . trait_ref . def_id } }
    };
}

impl_392!();