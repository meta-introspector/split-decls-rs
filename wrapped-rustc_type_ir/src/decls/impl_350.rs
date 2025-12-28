macro_rules! deps {
    () => {
        Interner!();
        TraitPredicate!();
        PredicatePolarity!();
        Binder!();
        Ty!();
    };
}

macro_rules! impl_350 {
    () => {
        deps!();
        impl < I : Interner > ty :: Binder < I , TraitPredicate < I > > { pub fn def_id (self) -> I :: TraitId { self . skip_binder () . def_id () } pub fn self_ty (self) -> ty :: Binder < I , I :: Ty > { self . map_bound (| trait_ref | trait_ref . self_ty ()) } # [inline] pub fn polarity (self) -> PredicatePolarity { self . skip_binder () . polarity } }
    };
}

impl_350!()