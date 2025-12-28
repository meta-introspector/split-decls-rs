macro_rules! deps {
    () => {
        SpanDecoder!();
    };
}

macro_rules! impl_63 {
    () => {
        deps!();
        impl < D : SpanDecoder > Decodable < D > for DefId { fn decode (s : & mut D) -> DefId { s . decode_def_id () } }
    };
}

impl_63!()