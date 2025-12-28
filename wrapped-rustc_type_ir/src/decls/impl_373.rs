macro_rules! deps {
    () => {
        AliasTyKind!();
        AliasTermKind!();
    };
}

macro_rules! impl_373 {
    () => {
        deps!();
        impl From < ty :: AliasTyKind > for AliasTermKind { fn from (value : ty :: AliasTyKind) -> Self { match value { ty :: Projection => AliasTermKind :: ProjectionTy , ty :: Opaque => AliasTermKind :: OpaqueTy , ty :: Free => AliasTermKind :: FreeTy , ty :: Inherent => AliasTermKind :: InherentTy , } } }
    };
}

impl_373!();