macro_rules! deps {
    () => {
        Error!();
        DecodeBufferError!();
    };
}

macro_rules! impl_62 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl std :: error :: Error for DecodeBufferError { }
    };
}

impl_62!();