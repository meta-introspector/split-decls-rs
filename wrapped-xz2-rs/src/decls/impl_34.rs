macro_rules! deps {
    () => {
        XzEncoder!();
    };
}

macro_rules! impl_34 {
    () => {
        deps!();
        # [cfg (feature = "tokio")] impl < R : AsyncRead + BufRead > AsyncRead for XzEncoder < R > { }
    };
}

impl_34!();