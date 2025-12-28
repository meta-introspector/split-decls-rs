macro_rules! deps {
    () => {
        AbiHashStable!();
    };
}

macro_rules! default_hash_impl {
    () => {
        deps!();
        macro_rules ! default_hash_impl { ($ ($ t : ty ,) +) => { $ (impl <'tcx > AbiHashStable <'tcx > for $ t { # [inline] fn abi_hash (& self , _tcx : TyCtxt <'tcx >, hasher : & mut StableHasher) { :: std :: hash :: Hash :: hash (self , hasher) ; } }) * } ; }
    };
}

default_hash_impl!();