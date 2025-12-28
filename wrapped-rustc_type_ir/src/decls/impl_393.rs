macro_rules! deps {
    () => {
        Interner!();
        Binder!();
        Ty!();
        BoundConstness!();
        HostEffectPredicate!();
    };
}

macro_rules! impl_393 {
    () => {
        deps!();
        impl < I : Interner > ty :: Binder < I , HostEffectPredicate < I > > { pub fn def_id (self) -> I :: TraitId { self . skip_binder () . def_id () } pub fn self_ty (self) -> ty :: Binder < I , I :: Ty > { self . map_bound (| trait_ref | trait_ref . self_ty ()) } # [inline] pub fn constness (self) -> BoundConstness { self . skip_binder () . constness } }
    };
}

impl_393!();