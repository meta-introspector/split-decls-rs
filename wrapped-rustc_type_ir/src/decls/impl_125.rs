macro_rules! deps {
    () => {
        Relate!();
        Interner!();
        TypeRelation!();
        RelateResult!();
        Binder!();
    };
}

macro_rules! impl_125 {
    () => {
        deps!();
        impl < I : Interner , T : Relate < I > > Relate < I > for ty :: Binder < I , T > { fn relate < R : TypeRelation < I > > (relation : & mut R , a : ty :: Binder < I , T > , b : ty :: Binder < I , T > ,) -> RelateResult < I , ty :: Binder < I , T > > { relation . binders (a , b) } }
    };
}

impl_125!()