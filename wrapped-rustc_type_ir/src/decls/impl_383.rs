macro_rules! deps {
    () => {
        Interner!();
        Ty!();
        DefId!();
        ProjectionPredicate!();
    };
}

macro_rules! impl_383 {
    () => {
        deps!();
        impl < I : Interner > ProjectionPredicate < I > { pub fn self_ty (self) -> I :: Ty { self . projection_term . self_ty () } pub fn with_replaced_self_ty (self , interner : I , self_ty : I :: Ty) -> ProjectionPredicate < I > { Self { projection_term : self . projection_term . with_replaced_self_ty (interner , self_ty) , .. self } } pub fn trait_def_id (self , interner : I) -> I :: TraitId { self . projection_term . trait_def_id (interner) } pub fn def_id (self) -> I :: DefId { self . projection_term . def_id } }
    };
}

impl_383!();