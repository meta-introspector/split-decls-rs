macro_rules! deps {
    () => {
        ExpnId!();
        SpanDecoder!();
    };
}

macro_rules! impl_84 {
    () => {
        deps!();
        impl < D : SpanDecoder > Decodable < D > for LocalExpnId { fn decode (d : & mut D) -> Self { ExpnId :: expect_local (ExpnId :: decode (d)) } }
    };
}

impl_84!()