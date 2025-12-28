macro_rules! deps {
    () => {
        NonceGenerator!();
        StorageNonce!();
    };
}

macro_rules! NONCE {
    () => {
        deps!();
        # [cfg (not (feature = "inventory"))] static NONCE : crate :: nonce :: NonceGenerator < StorageNonce > = crate :: nonce :: NonceGenerator :: new () ;
    };
}

NONCE!()