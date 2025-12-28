macro_rules! ZipCryptoValidator {
    () => {
        pub enum ZipCryptoValidator { PkzipCrc32 (u32) , InfoZipMsdosTime (u16) , }
    };
}

ZipCryptoValidator!()