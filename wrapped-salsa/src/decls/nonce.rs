macro_rules! nonce {
    () => {
        # [cfg (not (feature = "inventory"))] mod nonce ;
    };
}

nonce!()