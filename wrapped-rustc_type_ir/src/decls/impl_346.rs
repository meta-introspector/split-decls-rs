macro_rules! deps {
    () => {
        Binder!();
        HostEffectPredicate!();
        BoundConstness!();
        Clause!();
        ClauseKind!();
        Interner!();
        Ty!();
        TraitRef!();
    };
}

macro_rules! impl_346 {
    () => {
        deps!();
        impl < I : Interner > ty :: Binder < I , TraitRef < I > > { pub fn self_ty (& self) -> ty :: Binder < I , I :: Ty > { self . map_bound_ref (| tr | tr . self_ty ()) } pub fn def_id (& self) -> I :: TraitId { self . skip_binder () . def_id } pub fn to_host_effect_clause (self , cx : I , constness : BoundConstness) -> I :: Clause { self . map_bound (| trait_ref | { ty :: ClauseKind :: HostEffect (HostEffectPredicate { trait_ref , constness }) }) . upcast (cx) } }
    };
}

impl_346!()