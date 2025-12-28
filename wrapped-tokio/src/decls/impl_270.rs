macro_rules! deps {
    () => {
        OwnedBuf!();
    };
}

macro_rules! impl_270 {
    () => {
        deps!();
        impl AsRef < [u8] > for OwnedBuf { fn as_ref (& self) -> & [u8] { match self { Self :: Vec (vec) => vec , # [cfg (feature = "io-util")] Self :: Bytes (bytes) => bytes , } } }
    };
}

impl_270!()