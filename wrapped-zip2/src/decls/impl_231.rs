macro_rules! deps {
    () => {
        MaybeEncrypted!();
    };
}

macro_rules! impl_231 {
    () => {
        deps!();
        impl < W : Write > Write for MaybeEncrypted < W > { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { match self { MaybeEncrypted :: Unencrypted (w) => w . write (buf) , # [cfg (feature = "aes-crypto")] MaybeEncrypted :: Aes (w) => w . write (buf) , MaybeEncrypted :: ZipCrypto (w) => w . write (buf) , } } fn flush (& mut self) -> io :: Result < () > { match self { MaybeEncrypted :: Unencrypted (w) => w . flush () , # [cfg (feature = "aes-crypto")] MaybeEncrypted :: Aes (w) => w . flush () , MaybeEncrypted :: ZipCrypto (w) => w . flush () , } } }
    };
}

impl_231!()