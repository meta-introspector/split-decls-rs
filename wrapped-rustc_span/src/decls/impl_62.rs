macro_rules! deps {
    () => {
        SpanDecoder!();
    };
}

macro_rules! impl_62 {
    () => {
        deps!();
        impl < D : SpanDecoder > Decodable < D > for DefIndex { fn decode (s : & mut D) -> DefIndex { s . decode_def_index () } }
    };
}

impl_62!()