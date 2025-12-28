macro_rules! deps {
    () => {
        SpanEncoder!();
    };
}

macro_rules! impl_52 {
    () => {
        deps!();
        impl < E : SpanEncoder > Encodable < E > for DefId { fn encode (& self , s : & mut E) { s . encode_def_id (* self) } }
    };
}

impl_52!()