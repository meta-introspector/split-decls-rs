macro_rules! deps {
    () => {
        AbiHashStable!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        impl < 'tcx > AbiHashStable < 'tcx > for ty :: FnSig < 'tcx > { fn abi_hash (& self , tcx : TyCtxt < 'tcx > , hasher : & mut StableHasher) { for ty in self . inputs_and_output { ty . abi_hash (tcx , hasher) ; } self . safety . is_safe () . abi_hash (tcx , hasher) ; } }
    };
}

impl_8!()