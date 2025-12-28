macro_rules! deps {
    () => {
        ZipCryptoReader!();
    };
}

macro_rules! ZipCryptoReaderValid {
    () => {
        deps!();
        # [doc = " A ZipCrypto reader with verified password"] pub struct ZipCryptoReaderValid < R > { reader : ZipCryptoReader < R > , }
    };
}

ZipCryptoReaderValid!()