macro_rules! deps {
    () => {
        Binder!();
        DefId!();
        Interner!();
        Term!();
        ProjectionPredicate!();
        TraitRef!();
    };
}

macro_rules! impl_384 {
    () => {
        deps!();
        impl < I : Interner > ty :: Binder < I , ProjectionPredicate < I > > { # [doc = " Returns the `DefId` of the trait of the associated item being projected."] # [inline] pub fn trait_def_id (& self , cx : I) -> I :: TraitId { self . skip_binder () . projection_term . trait_def_id (cx) } pub fn term (& self) -> ty :: Binder < I , I :: Term > { self . map_bound (| predicate | predicate . term) } # [doc = " The `DefId` of the `TraitItem` for the associated type."] # [doc = ""] # [doc = " Note that this is not the `DefId` of the `TraitRef` containing this"] # [doc = " associated type, which is in `tcx.associated_item(projection_def_id()).container`."] pub fn item_def_id (& self) -> I :: DefId { self . skip_binder () . projection_term . def_id } }
    };
}

impl_384!();