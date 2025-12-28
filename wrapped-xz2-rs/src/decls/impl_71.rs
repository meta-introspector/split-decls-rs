macro_rules! deps {
    () => {
        XzDecoder!();
    };
}

macro_rules! impl_71 {
    () => {
        deps!();
        # [cfg (feature = "tokio")] impl < W : AsyncRead + AsyncWrite > AsyncRead for XzDecoder < W > { }
    };
}

impl_71!()