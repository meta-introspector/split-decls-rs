macro_rules! deps {
    () => {
        Binder!();
        ProjectionPredicate!();
        Interner!();
        ExistentialProjection!();
        DefId!();
        Ty!();
    };
}

macro_rules! impl_370 {
    () => {
        deps!();
        impl < I : Interner > ty :: Binder < I , ExistentialProjection < I > > { pub fn with_self_ty (& self , cx : I , self_ty : I :: Ty) -> ty :: Binder < I , ProjectionPredicate < I > > { self . map_bound (| p | p . with_self_ty (cx , self_ty)) } pub fn item_def_id (& self) -> I :: DefId { self . skip_binder () . def_id } }
    };
}

impl_370!();