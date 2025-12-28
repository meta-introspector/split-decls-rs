macro_rules! deps {
    () => {
        AbiHashStable!();
    };
}

macro_rules! impl_5 {
    () => {
        deps!();
        impl < 'tcx > AbiHashStable < 'tcx > for Symbol { # [inline] fn abi_hash (& self , tcx : TyCtxt < 'tcx > , hasher : & mut StableHasher) { self . as_str () . abi_hash (tcx , hasher) ; } }
    };
}

impl_5!();