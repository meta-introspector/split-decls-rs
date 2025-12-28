macro_rules! deps {
    () => {
        ZipCryptoReaderValid!();
    };
}

macro_rules! impl_283 {
    () => {
        deps!();
        impl < R : std :: io :: Read > ZipCryptoReaderValid < R > { # [doc = " Consumes this decoder, returning the underlying reader."] pub fn into_inner (self) -> R { self . reader . file } }
    };
}

impl_283!();