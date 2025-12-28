macro_rules! AbiHashStable {
    () => {
        trait AbiHashStable < 'tcx > { fn abi_hash (& self , tcx : TyCtxt < 'tcx > , hasher : & mut StableHasher) ; }
    };
}

AbiHashStable!()