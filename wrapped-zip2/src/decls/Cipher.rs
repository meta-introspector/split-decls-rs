macro_rules! deps {
    () => {
        AesCtrZipKeyStream!();
        Aes128!();
        Aes256!();
        Aes192!();
    };
}

macro_rules! Cipher {
    () => {
        deps!();
        enum Cipher { Aes128 (Box < aes_ctr :: AesCtrZipKeyStream < aes_ctr :: Aes128 > >) , Aes192 (Box < aes_ctr :: AesCtrZipKeyStream < aes_ctr :: Aes192 > >) , Aes256 (Box < aes_ctr :: AesCtrZipKeyStream < aes_ctr :: Aes256 > >) , }
    };
}

Cipher!()