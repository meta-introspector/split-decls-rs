macro_rules! deps {
    () => {
        LocalDefId!();
        SpanDecoder!();
        DefId!();
    };
}

macro_rules! impl_121 {
    () => {
        deps!();
        impl < D : SpanDecoder > Decodable < D > for LocalDefId { fn decode (d : & mut D) -> LocalDefId { DefId :: decode (d) . expect_local () } }
    };
}

impl_121!()