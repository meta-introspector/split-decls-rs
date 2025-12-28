macro_rules! deps {
    () => {
        Interner!();
        Ty!();
        TraitPredicate!();
    };
}

macro_rules! impl_349 {
    () => {
        deps!();
        impl < I : Interner > TraitPredicate < I > { pub fn with_replaced_self_ty (self , interner : I , self_ty : I :: Ty) -> Self { Self { trait_ref : self . trait_ref . with_replaced_self_ty (interner , self_ty) , polarity : self . polarity , } } pub fn def_id (self) -> I :: TraitId { self . trait_ref . def_id } pub fn self_ty (self) -> I :: Ty { self . trait_ref . self_ty () } }
    };
}

impl_349!()