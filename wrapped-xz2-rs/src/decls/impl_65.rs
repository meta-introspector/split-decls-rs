macro_rules! deps {
    () => {
        XzEncoder!();
    };
}

macro_rules! impl_65 {
    () => {
        deps!();
        # [cfg (feature = "tokio")] impl < W : AsyncRead + AsyncWrite > AsyncRead for XzEncoder < W > { }
    };
}

impl_65!();