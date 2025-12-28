macro_rules! deps {
    () => {
        AbiHashStable!();
    };
}

macro_rules! impl_6 {
    () => {
        deps!();
        impl < 'tcx , T : AbiHashStable < 'tcx > > AbiHashStable < 'tcx > for [T] { fn abi_hash (& self , tcx : TyCtxt < 'tcx > , hasher : & mut StableHasher) { self . len () . abi_hash (tcx , hasher) ; for item in self { item . abi_hash (tcx , hasher) ; } } }
    };
}

impl_6!();