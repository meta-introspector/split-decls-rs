macro_rules! deps {
    () => {
        ZipCryptoKeys!();
    };
}

macro_rules! ZipCryptoWriter {
    () => {
        deps!();
        # [allow (unused)] pub (crate) struct ZipCryptoWriter < W > { pub (crate) writer : W , pub (crate) buffer : Vec < u8 > , pub (crate) keys : ZipCryptoKeys , }
    };
}

ZipCryptoWriter!();