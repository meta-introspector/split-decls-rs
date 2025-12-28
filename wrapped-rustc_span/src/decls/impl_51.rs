macro_rules! deps {
    () => {
        SpanEncoder!();
    };
}

macro_rules! impl_51 {
    () => {
        deps!();
        impl < E : SpanEncoder > Encodable < E > for DefIndex { fn encode (& self , s : & mut E) { s . encode_def_index (* self) } }
    };
}

impl_51!()