macro_rules! deps {
    () => {
        CompressionMethod!();
    };
}

macro_rules! impl_34 {
    () => {
        deps!();
        impl Default for CompressionMethod { fn default () -> Self { # [cfg (feature = "_deflate-any")] return CompressionMethod :: Deflated ; # [cfg (not (feature = "_deflate-any"))] return CompressionMethod :: Stored ; } }
    };
}

impl_34!();