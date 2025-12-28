macro_rules! deps {
    () => {
        AbiHashStable!();
    };
}

macro_rules! impl_3 {
    () => {
        deps!();
        impl < 'tcx > AbiHashStable < 'tcx > for bool { # [inline] fn abi_hash (& self , tcx : TyCtxt < 'tcx > , hasher : & mut StableHasher) { (if * self { 1u8 } else { 0u8 }) . abi_hash (tcx , hasher) ; } }
    };
}

impl_3!()