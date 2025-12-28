macro_rules! deps {
    () => {
        SpanEncoder!();
    };
}

macro_rules! impl_277 {
    () => {
        deps!();
        impl < E : SpanEncoder > Encodable < E > for AttrId { fn encode (& self , _s : & mut E) { } }
    };
}

impl_277!()