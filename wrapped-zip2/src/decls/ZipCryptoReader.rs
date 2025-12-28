macro_rules! deps {
    () => {
        ZipCryptoKeys!();
    };
}

macro_rules! ZipCryptoReader {
    () => {
        deps!();
        # [doc = " A ZipCrypto reader with unverified password"] pub struct ZipCryptoReader < R > { file : R , keys : ZipCryptoKeys , }
    };
}

ZipCryptoReader!()