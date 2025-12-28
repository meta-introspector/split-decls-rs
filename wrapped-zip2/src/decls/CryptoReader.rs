macro_rules! deps {
    () => {
        AesReaderValid!();
        AesVendorVersion!();
        ZipCryptoReaderValid!();
    };
}

macro_rules! CryptoReader {
    () => {
        deps!();
        # [allow (clippy :: large_enum_variant)] pub (crate) enum CryptoReader < 'a , R : Read > { Plaintext (io :: Take < & 'a mut R >) , ZipCrypto (ZipCryptoReaderValid < io :: Take < & 'a mut R > >) , # [cfg (feature = "aes-crypto")] Aes { reader : AesReaderValid < io :: Take < & 'a mut R > > , vendor_version : AesVendorVersion , } , }
    };
}

CryptoReader!();