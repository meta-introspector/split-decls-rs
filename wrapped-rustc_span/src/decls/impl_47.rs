macro_rules! deps {
    () => {
        SpanEncoder!();
    };
}

macro_rules! impl_47 {
    () => {
        deps!();
        impl < E : SpanEncoder > Encodable < E > for ByteSymbol { fn encode (& self , s : & mut E) { s . encode_byte_symbol (* self) ; } }
    };
}

impl_47!()