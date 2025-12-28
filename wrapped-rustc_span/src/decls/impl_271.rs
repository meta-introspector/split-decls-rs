macro_rules! deps {
    () => {
        ByteSymbol!();
        SpanEncoder!();
    };
}

macro_rules! impl_271 {
    () => {
        deps!();
        impl < E : SpanEncoder > Encodable < E > for ByteSymbol { fn encode (& self , s : & mut E) { s . encode_byte_symbol (* self) ; } }
    };
}

impl_271!()