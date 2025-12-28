macro_rules! deps {
    () => {
        AbiHashStable!();
    };
}

macro_rules! impl_4 {
    () => {
        deps!();
        impl < 'tcx > AbiHashStable < 'tcx > for str { # [inline] fn abi_hash (& self , tcx : TyCtxt < 'tcx > , hasher : & mut StableHasher) { self . as_bytes () . abi_hash (tcx , hasher) ; } }
    };
}

impl_4!()