macro_rules! deps {
    () => {
        Nonce!();
    };
}

macro_rules! StorageNonce {
    () => {
        deps!();
        # [doc = " Nonce type representing the underlying database storage."] # [derive (Copy , Clone , PartialEq , Eq , PartialOrd , Ord)] # [cfg (not (feature = "inventory"))] pub struct StorageNonce ;
    };
}

StorageNonce!();