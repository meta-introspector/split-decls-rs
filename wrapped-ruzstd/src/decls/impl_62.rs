macro_rules! deps {
    () => {
        DecodeBufferError!();
        Error!();
    };
}

macro_rules! impl_62 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl std :: error :: Error for DecodeBufferError { }
    };
}

impl_62!()