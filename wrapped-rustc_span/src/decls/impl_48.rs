macro_rules! deps {
    () => {
        SpanEncoder!();
    };
}

macro_rules! impl_48 {
    () => {
        deps!();
        impl < E : SpanEncoder > Encodable < E > for ExpnId { fn encode (& self , s : & mut E) { s . encode_expn_id (* self) } }
    };
}

impl_48!()