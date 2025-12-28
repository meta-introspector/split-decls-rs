macro_rules! deps {
    () => {
        Aes192!();
        Aes128!();
        AesCtrZipKeyStream!();
        Aes256!();
    };
}

macro_rules! Cipher {
    () => {
        deps!();
        enum Cipher { Aes128 (Box < aes_ctr :: AesCtrZipKeyStream < aes_ctr :: Aes128 > >) , Aes192 (Box < aes_ctr :: AesCtrZipKeyStream < aes_ctr :: Aes192 > >) , Aes256 (Box < aes_ctr :: AesCtrZipKeyStream < aes_ctr :: Aes256 > >) , }
    };
}

Cipher!();