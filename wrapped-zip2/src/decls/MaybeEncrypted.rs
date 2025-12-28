macro_rules! deps {
    () => {
        AesWriter!();
        ZipCryptoWriter!();
    };
}

macro_rules! MaybeEncrypted {
    () => {
        deps!();
        enum MaybeEncrypted < W > { Unencrypted (W) , # [cfg (feature = "aes-crypto")] Aes (AesWriter < W >) , ZipCrypto (crate :: zipcrypto :: ZipCryptoWriter < W >) , }
    };
}

MaybeEncrypted!()