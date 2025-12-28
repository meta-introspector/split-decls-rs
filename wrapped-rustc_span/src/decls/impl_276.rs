macro_rules! deps {
    () => {
        SpanEncoder!();
        DefId!();
    };
}

macro_rules! impl_276 {
    () => {
        deps!();
        impl < E : SpanEncoder > Encodable < E > for DefId { fn encode (& self , s : & mut E) { s . encode_def_id (* self) } }
    };
}

impl_276!();