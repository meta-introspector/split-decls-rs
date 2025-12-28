macro_rules! deps {
    () => {
        AbiHashStable!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl < 'tcx > AbiHashStable < 'tcx > for ty :: GenericArgKind < 'tcx > { fn abi_hash (& self , tcx : TyCtxt < 'tcx > , hasher : & mut StableHasher) { match self { ty :: GenericArgKind :: Type (t) => t . abi_hash (tcx , hasher) , ty :: GenericArgKind :: Lifetime (_) | ty :: GenericArgKind :: Const (_) => unimplemented ! () , } } }
    };
}

impl_10!()