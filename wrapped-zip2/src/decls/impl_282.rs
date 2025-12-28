macro_rules! deps {
    () => {
        ZipCryptoReaderValid!();
    };
}

macro_rules! impl_282 {
    () => {
        deps!();
        impl < R : std :: io :: Read > std :: io :: Read for ZipCryptoReaderValid < R > { fn read (& mut self , buf : & mut [u8]) -> std :: io :: Result < usize > { let n = self . reader . file . read (buf) ? ; for byte in buf . iter_mut () . take (n) { * byte = self . reader . keys . decrypt_byte (* byte) ; } Ok (n) } }
    };
}

impl_282!();