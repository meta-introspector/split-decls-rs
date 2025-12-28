macro_rules! deps {
    () => {
        XzDecoder!();
    };
}

macro_rules! impl_40 {
    () => {
        deps!();
        # [cfg (feature = "tokio")] impl < R : AsyncRead + BufRead > AsyncRead for XzDecoder < R > { }
    };
}

impl_40!()