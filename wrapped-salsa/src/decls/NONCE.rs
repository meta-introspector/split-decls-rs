macro_rules! deps {
    () => {
        StorageNonce!();
        NonceGenerator!();
    };
}

macro_rules! NONCE {
    () => {
        deps!();
        # [cfg (not (feature = "inventory"))] static NONCE : crate :: nonce :: NonceGenerator < StorageNonce > = crate :: nonce :: NonceGenerator :: new () ;
    };
}

NONCE!();