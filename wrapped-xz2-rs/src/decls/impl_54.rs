macro_rules! deps {
    () => {
        XzDecoder!();
    };
}

macro_rules! impl_54 {
    () => {
        deps!();
        # [cfg (feature = "tokio")] impl < R : AsyncRead + Read > AsyncRead for XzDecoder < R > { }
    };
}

impl_54!();