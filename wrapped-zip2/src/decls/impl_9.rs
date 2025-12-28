macro_rules! deps {
    () => {
        AesReaderValid!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        impl < R : Read > AesReaderValid < R > { # [doc = " Consumes this decoder, returning the underlying reader."] pub fn into_inner (self) -> R { self . reader } }
    };
}

impl_9!();