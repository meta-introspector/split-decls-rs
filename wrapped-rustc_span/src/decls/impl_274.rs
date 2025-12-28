macro_rules! deps {
    () => {
        SpanEncoder!();
    };
}

macro_rules! impl_274 {
    () => {
        deps!();
        impl < E : SpanEncoder > Encodable < E > for CrateNum { fn encode (& self , s : & mut E) { s . encode_crate_num (* self) } }
    };
}

impl_274!();