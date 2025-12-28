macro_rules! deps {
    () => {
        AbiHashStable!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        impl < 'tcx > AbiHashStable < 'tcx > for ty :: GenericArg < 'tcx > { fn abi_hash (& self , tcx : TyCtxt < 'tcx > , hasher : & mut StableHasher) { self . kind () . abi_hash (tcx , hasher) ; } }
    };
}

impl_9!();