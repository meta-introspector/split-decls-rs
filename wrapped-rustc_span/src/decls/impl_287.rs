macro_rules! deps {
    () => {
        DefId!();
        SpanDecoder!();
    };
}

macro_rules! impl_287 {
    () => {
        deps!();
        impl < D : SpanDecoder > Decodable < D > for DefId { fn decode (s : & mut D) -> DefId { s . decode_def_id () } }
    };
}

impl_287!();