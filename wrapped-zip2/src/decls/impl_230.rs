macro_rules! deps {
    () => {
        MaybeEncrypted!();
    };
}

macro_rules! impl_230 {
    () => {
        deps!();
        impl < W > Debug for MaybeEncrypted < W > { fn fmt (& self , f : & mut Formatter < '_ >) -> std :: fmt :: Result { f . write_str (match self { MaybeEncrypted :: Unencrypted (_) => "Unencrypted" , # [cfg (feature = "aes-crypto")] MaybeEncrypted :: Aes (_) => "AES" , MaybeEncrypted :: ZipCrypto (_) => "ZipCrypto" , }) } }
    };
}

impl_230!();