macro_rules! deps {
    () => {
        Interner!();
        Ty!();
        NormalizesTo!();
        DefId!();
    };
}

macro_rules! impl_388 {
    () => {
        deps!();
        impl < I : Interner > NormalizesTo < I > { pub fn self_ty (self) -> I :: Ty { self . alias . self_ty () } pub fn with_replaced_self_ty (self , interner : I , self_ty : I :: Ty) -> NormalizesTo < I > { Self { alias : self . alias . with_replaced_self_ty (interner , self_ty) , .. self } } pub fn trait_def_id (self , interner : I) -> I :: TraitId { self . alias . trait_def_id (interner) } pub fn def_id (self) -> I :: DefId { self . alias . def_id } }
    };
}

impl_388!()