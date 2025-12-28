macro_rules! deps {
    () => {
        ZipCryptoWriter!();
    };
}

macro_rules! impl_279 {
    () => {
        deps!();
        impl < W : std :: io :: Write > ZipCryptoWriter < W > { # [allow (unused)] pub (crate) fn finish (mut self , crc32 : u32) -> std :: io :: Result < W > { self . buffer [11] = (crc32 >> 24) as u8 ; for byte in self . buffer . iter_mut () { * byte = self . keys . encrypt_byte (* byte) ; } self . writer . write_all (& self . buffer) ? ; self . writer . flush () ? ; Ok (self . writer) } }
    };
}

impl_279!();