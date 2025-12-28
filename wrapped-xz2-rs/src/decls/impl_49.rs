macro_rules! deps {
    () => {
        XzEncoder!();
    };
}

macro_rules! impl_49 {
    () => {
        deps!();
        # [cfg (feature = "tokio")] impl < R : AsyncRead > AsyncRead for XzEncoder < R > { }
    };
}

impl_49!()